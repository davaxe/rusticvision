use std::borrow::Cow;
use std::io::Write;
use std::num::{NonZeroU32, NonZeroU64};
use std::sync::{atomic::AtomicU32, Mutex};
use wgpu::util::DeviceExt;

use crate::data_structures::BoundingBoxData;
use crate::{primitives::Ray, traits::Intersectable};

use super::{Camera, Scene};

use glam::Vec3A;
use image::RgbImage;
use itertools::Itertools;
use rayon::prelude::*;

pub struct SceneRenderer<'scene> {
    camera: &'scene Camera,
    scene: &'scene Scene,
    sample_count: u32,
    recursion_depth: u32,
}

impl<'scene> SceneRenderer<'scene> {
    #[inline]
    pub fn new(camera: &'scene Camera, scene: &'scene Scene) -> Self {
        Self {
            camera,
            scene,
            sample_count: 1,
            recursion_depth: 1,
        }
    }

    /// Sets the number of samples to take per pixel.
    #[inline]
    pub fn set_sample_count(&mut self, sample_count: u32) {
        self.sample_count = sample_count;
    }

    /// Sets the recursion depth for the ray tracer.
    #[inline]
    pub fn set_recursion_depth(&mut self, recursion_depth: u32) {
        self.recursion_depth = recursion_depth;
    }

    pub fn render(&self) -> RgbImage {
        let (width, height) = self.camera.get_dimensions();
        let update_count = width * height / 100;
        let progress = AtomicU32::new(0);
        let image: Mutex<image::RgbImage> = Mutex::new(RgbImage::new(width, height));
        (0..width)
            .cartesian_product(0..height)
            .par_bridge()
            .for_each(|(x, y)| {
                let pixel = self.render_pixel(x, y);
                let mut image = image.lock().unwrap();
                image.put_pixel(x, y, Self::vec3_to_rgb(pixel));
                let progress = progress.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
                if progress % update_count == 0 {
                    print!("\rProgress: {}%", progress / update_count);
                    std::io::stdout().flush().unwrap();
                }
            });
        image.into_inner().unwrap()
    }

    #[inline]
    fn render_pixel(&self, x: u32, y: u32) -> Vec3A {
        (0..self.sample_count)
            .map(|_| self.trace(&self.camera.get_jittered_ray(x, y), 0, Vec3A::ONE))
            .sum::<Vec3A>()
            / self.sample_count as f32
    }

    fn trace(&self, ray: &Ray, depth: u32, throughput: Vec3A) -> Vec3A {
        if depth > self.recursion_depth {
            return Vec3A::ZERO;
        }

        let mesh = self.scene.triangle_mesh();
        let mut throughput: Vec3A = throughput;
        let mut color = Vec3A::ZERO;

        if let Some(hit) = self.scene.intersect(ray, 0.01, 100.0) {
            let material = hit.material(mesh);
            color += material.emissive_color * throughput * 5.0;
            throughput *= material.diffuse_color;
            color += self.trace(&hit.random_outgoing_ray(mesh), depth + 1, throughput);
        }
        color
    }

    #[inline]
    fn vec3_to_rgb(color: Vec3A) -> image::Rgb<u8> {
        let r = (color.x * 255.0) as u8;
        let g = (color.y * 255.0) as u8;
        let b = (color.z * 255.0) as u8;
        image::Rgb([r, g, b])
    }
}

pub struct GPUSceneRenderer<'scene> {
    camera: &'scene Camera,
    scene: &'scene Scene,
    sample_count: u32,
    recursion_depth: u32,
}

impl<'scene> GPUSceneRenderer<'scene> {
    #[inline]
    pub fn new(camera: &'scene Camera, scene: &'scene Scene) -> Self {
        Self {
            camera,
            scene,
            sample_count: 1,
            recursion_depth: 1,
        }
    }

    /// Sets the number of samples to take per pixel.
    #[inline]
    pub fn set_sample_count(&mut self, sample_count: u32) {
        self.sample_count = sample_count;
    }

    /// Sets the recursion depth for the ray tracer.
    #[inline]
    pub fn set_recursion_depth(&mut self, recursion_depth: u32) {
        self.recursion_depth = recursion_depth;
    }

    pub async fn render(&self) -> Option<Vec<[f32; 4]>> {
        // Instantiates instance of WebGPU
        let instance = wgpu::Instance::default();

        // `request_adapter` instantiates the general connection to the GPU
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions::default())
            .await?;

        let limits = wgpu::Limits {
            max_dynamic_storage_buffers_per_pipeline_layout: 9,
            ..Default::default()
        };

        // `request_device` instantiates the feature specific connection to the GPU, defining some parameters,
        //  `features` being the available features.
        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    label: None,
                    features: wgpu::Features::empty(),
                    limits,
                },
                None,
            )
            .await
            .unwrap();
        self.render_gpu_inner(&device, &queue).await
    }

    async fn render_gpu_inner(
        &self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
    ) -> Option<Vec<[f32; 4]>> {
        // Loads the shader from WGSL
        let cs_module = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Main shader: raytrace.wgsl"),
            source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!(
                "../../assets/shaders/raytrace.wgsl"
            ))),
        });

        let (width, height) = self.camera.get_dimensions();
        let size = (width as usize * height as usize * 4 * std::mem::size_of::<f32>())
            as wgpu::BufferAddress;

        let staging_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: None,
            size,
            usage: wgpu::BufferUsages::MAP_READ | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        let vertex_position_buffer = self.vertex_position_buffer(device);
        let vertex_normal_buffer = self.vertex_normal_buffer(device);
        let triangle_index_buffer = self.triangle_index_buffer(device);
        let (obj_buffer, aabb_buffer) = self.object_buffer(device);
        let pixel_buffer = self.pixel_buffer(device);
        let camera_data_buffer = self.camera_data_buffer(device);
        let random_buffer = self.random_numbers_buffer(device);
        let material_buffer = self.material_buffer(device);

        let compute_pipeline = device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
            label: Some("Compute pipeline"),
            layout: None,
            module: &cs_module,
            entry_point: "main",
        });

        let bind_group_layout = compute_pipeline.get_bind_group_layout(0);
        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: None,
            layout: &bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: vertex_position_buffer.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: vertex_normal_buffer.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 2,
                    resource: triangle_index_buffer.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 3,
                    resource: aabb_buffer.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 4,
                    resource: obj_buffer.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 5,
                    resource: pixel_buffer.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 6,
                    resource: material_buffer.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 7,
                    resource: random_buffer.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 8,
                    resource: camera_data_buffer.as_entire_binding(),
                },
            ],
        });

        let mut encoder =
            device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });
        {
            let mut cpass =
                encoder.begin_compute_pass(&wgpu::ComputePassDescriptor { label: None });
            cpass.set_pipeline(&compute_pipeline);
            cpass.set_bind_group(0, &bind_group, &[]);
            cpass.insert_debug_marker("Ray trace test");
            cpass.dispatch_workgroups(width * height / 145 + 1, 1, 1);
        }

        encoder.copy_buffer_to_buffer(&pixel_buffer, 0, &staging_buffer, 0, size);

        queue.submit(Some(encoder.finish()));

        let buffer_slice = staging_buffer.slice(..);
        let (sender, receiver) = futures_intrusive::channel::shared::oneshot_channel();
        buffer_slice.map_async(wgpu::MapMode::Read, move |v| sender.send(v).unwrap());

        device.poll(wgpu::Maintain::Wait);

        if let Some(Ok(())) = receiver.receive().await {
            // Gets contents of buffer
            let data = buffer_slice.get_mapped_range();

            // Since contents are got in bytes, this converts these bytes back to u32
            let result = bytemuck::cast_slice(&data).to_vec();

            // With the current interface, we have to make sure all mapped views are
            // dropped before we unmap the buffer.
            drop(data);
            staging_buffer.unmap(); // Unmaps buffer from memory
                                    // If you are familiar with C++ these 2 lines can be thought of similarly to:
                                    //   delete myPointer;
                                    //   myPointer = NULL;
                                    // It effectively frees the memory

            // Returns data from buffer
            Some(result)
        } else {
            panic!("failed to run compute on gpu!")
        }
    }

    fn vertex_position_buffer(&self, device: &wgpu::Device) -> wgpu::Buffer {
        Self::create_buffer(
            Some("Vertex position buffer"),
            device,
            bytemuck::cast_slice(self.scene.gpu_vertex_pos_data().as_slice()),
            wgpu::BufferUsages::STORAGE
                | wgpu::BufferUsages::COPY_DST
                | wgpu::BufferUsages::COPY_SRC,
        )
    }

    fn vertex_normal_buffer(&self, device: &wgpu::Device) -> wgpu::Buffer {
        Self::create_buffer(
            Some("Vertex position buffer"),
            device,
            bytemuck::cast_slice(self.scene.gpu_triangle_normal_data().as_slice()),
            wgpu::BufferUsages::STORAGE
                | wgpu::BufferUsages::COPY_DST
                | wgpu::BufferUsages::COPY_SRC,
        )
    }

    fn triangle_index_buffer(&self, device: &wgpu::Device) -> wgpu::Buffer {
        Self::create_buffer(
            Some("Triangle index buffer"),
            device,
            bytemuck::cast_slice(self.scene.gpu_triangle_index_data().as_slice()),
            wgpu::BufferUsages::STORAGE
                | wgpu::BufferUsages::COPY_DST
                | wgpu::BufferUsages::COPY_SRC,
        )
    }

    fn material_buffer(&self, device: &wgpu::Device) -> wgpu::Buffer {
        Self::create_buffer(
            Some("Material buffer"),
            device,
            bytemuck::cast_slice(self.scene.gpu_material_data().as_slice()),
            wgpu::BufferUsages::STORAGE
                | wgpu::BufferUsages::COPY_DST
                | wgpu::BufferUsages::COPY_SRC,
        )
    }

    fn object_buffer(&self, device: &wgpu::Device) -> (wgpu::Buffer, wgpu::Buffer) {
        let mut object_vec: Vec<u32> = Vec::new();
        let mut aabb_vec = Vec::new();

        object_vec.push(self.scene.objects.len() as u32);

        for (i, obj) in self.scene.objects.iter().enumerate() {
            object_vec.extend([
                i as u32,
                obj.triangle_start_index as u32,
                obj.triangle_count as u32,
            ]);
            let (min, max) = obj.bounding_box.bounds();
            let values = [min.x, min.y, min.z, 0f32, max.x, max.y, max.z, 0f32]; // Zeroes are padding
            aabb_vec.push(values);
        }
        let object_buffer = Self::create_buffer(
            Some("Object buffer"),
            device,
            bytemuck::cast_slice(object_vec.as_slice()),
            wgpu::BufferUsages::STORAGE
                | wgpu::BufferUsages::COPY_DST
                | wgpu::BufferUsages::COPY_SRC,
        );

        let aabb_buffer = Self::create_buffer(
            Some("AABB buffer"),
            device,
            bytemuck::cast_slice(aabb_vec.as_slice()),
            wgpu::BufferUsages::STORAGE
                | wgpu::BufferUsages::COPY_DST
                | wgpu::BufferUsages::COPY_SRC,
        );

        (object_buffer, aabb_buffer)
    }

    fn pixel_buffer(&self, device: &wgpu::Device) -> wgpu::Buffer {
        let (x, y) = self.camera.get_dimensions();
        let pixels = (0..x)
            .cartesian_product(0..y)
            .flat_map(|_| [1f32, 1f32, 1f32, 1f32])
            .collect_vec();

        Self::create_buffer(
            Some("Pixels buffer"),
            device,
            bytemuck::cast_slice(pixels.as_slice()),
            wgpu::BufferUsages::STORAGE
                | wgpu::BufferUsages::COPY_DST
                | wgpu::BufferUsages::COPY_SRC,
        )
    }

    fn camera_data_buffer(&self, device: &wgpu::Device) -> wgpu::Buffer {
        let inverse_view = self.camera.get_inverse_view().to_cols_array();
        let inverse_projection = self.camera.get_inverse_projection().to_cols_array();
        let position = self.camera.get_position().to_array();
        // vec4 is used in shader, to make sure the data is aligned correctly
        let position = [position[0], position[1], position[2], 0f32];
        let mut data = position
            .iter()
            .chain(inverse_projection.iter())
            .chain(inverse_view.iter())
            .flat_map(bytemuck::bytes_of)
            .copied()
            .collect_vec();

        let (width, height) = self.camera.get_dimensions();
        data.extend(bytemuck::bytes_of(&width));
        data.extend(bytemuck::bytes_of(&height));
        data.extend(bytemuck::bytes_of(&self.sample_count));
        data.extend(bytemuck::bytes_of(&self.recursion_depth));

        Self::create_buffer(
            Some("Pixels buffer"),
            device,
            bytemuck::cast_slice(data.as_slice()),
            wgpu::BufferUsages::UNIFORM
                | wgpu::BufferUsages::COPY_DST
                | wgpu::BufferUsages::COPY_SRC,
        )
    }

    fn random_numbers_buffer(&self, device: &wgpu::Device) -> wgpu::Buffer {
        let (x, y) = self.camera.get_dimensions();
        let pixels = x * y;
        let data = (0..pixels).map(|_| rand::random::<u32>()).collect_vec();
        Self::create_buffer(
            Some("Random values"),
            device,
            bytemuck::cast_slice(data.as_slice()),
            wgpu::BufferUsages::STORAGE
                | wgpu::BufferUsages::COPY_DST
                | wgpu::BufferUsages::COPY_SRC,
        )
    }

    fn create_buffer(
        label: Option<&str>,
        device: &wgpu::Device,
        contents: &[u8],
        usage: wgpu::BufferUsages,
    ) -> wgpu::Buffer {
        device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label,
            contents,
            usage,
        })
    }
}
