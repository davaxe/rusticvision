use std::borrow::Cow;

use image::RgbImage;
use itertools::Itertools;
use nom::ExtendInto;
use wgpu::util::DeviceExt;

use super::data_structures::*;

pub struct Renderer {
    gpu_data: GPUData,
    image_resolution: (u32, u32),
}

impl Renderer {
    pub fn new(gpu_data: GPUData, image_resolution: (u32, u32)) -> Self {
        Self {
            gpu_data,
            image_resolution,
        }
    }

    pub async fn render(&self) -> Option<RgbImage> {
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
    ) -> Option<RgbImage> {
        // Loads the shader from WGSL
        let cs_module = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Main shader: raytrace.wgsl"),
            source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!(
                "../assets/shaders/raytrace.wgsl"
            ))),
        });

        let (width, height) = self.image_resolution;
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
        let render_settings_buffer = self.render_settings_buffer(device);

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
                wgpu::BindGroupEntry {
                    binding: 9,
                    resource: render_settings_buffer.as_entire_binding(),
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
            Some(self.to_image(result))
        } else {
            panic!("failed to run compute on gpu!")
        }
    }

    fn to_image(&self, data: Vec<[f32; 4]>) -> RgbImage {
        let (width, height) = self.image_resolution;
        let mut image = RgbImage::new(width, height);
        (0..width).cartesian_product(0..height).for_each(|(x, y)| {
            let index = (x + (width - y - 1) * width) as usize;
            let [r, g, b, _] = data[index];
            let r = (r * 255.0) as u8;
            let g = (g * 255.0) as u8;
            let b = (b * 255.0) as u8;
            image.put_pixel(x, y, image::Rgb([r, g, b]));
        });
        image
    }

    fn vertex_position_buffer(&self, device: &wgpu::Device) -> wgpu::Buffer {
        Self::create_buffer(
            Some("Vertex position buffer"),
            device,
            self.gpu_data.to_bytes().vertex_positions,
            wgpu::BufferUsages::STORAGE
                | wgpu::BufferUsages::COPY_DST
                | wgpu::BufferUsages::COPY_SRC,
        )
    }

    fn vertex_normal_buffer(&self, device: &wgpu::Device) -> wgpu::Buffer {
        Self::create_buffer(
            Some("Vertex position buffer"),
            device,
            self.gpu_data.to_bytes().vertex_normals,
            wgpu::BufferUsages::STORAGE
                | wgpu::BufferUsages::COPY_DST
                | wgpu::BufferUsages::COPY_SRC,
        )
    }

    fn triangle_index_buffer(&self, device: &wgpu::Device) -> wgpu::Buffer {
        Self::create_buffer(
            Some("Triangle index buffer"),
            device,
            self.gpu_data.to_bytes().triangles,
            wgpu::BufferUsages::STORAGE
                | wgpu::BufferUsages::COPY_DST
                | wgpu::BufferUsages::COPY_SRC,
        )
    }

    fn material_buffer(&self, device: &wgpu::Device) -> wgpu::Buffer {
        Self::create_buffer(
            Some("Material buffer"),
            device,
            self.gpu_data.to_bytes().materials,
            wgpu::BufferUsages::STORAGE
                | wgpu::BufferUsages::COPY_DST
                | wgpu::BufferUsages::COPY_SRC,
        )
    }

    fn object_buffer(&self, device: &wgpu::Device) -> (wgpu::Buffer, wgpu::Buffer) {
        let count = self.gpu_data.objects.len() as u32;
        let first = bytemuck::bytes_of(&count);
        let second = self.gpu_data.to_bytes().objects;
        let object_data = [first, second].concat();

        let object_buffer = Self::create_buffer(
            Some("Object buffer"),
            device,
            object_data.as_slice(),
            wgpu::BufferUsages::STORAGE
                | wgpu::BufferUsages::COPY_DST
                | wgpu::BufferUsages::COPY_SRC,
        );

        let aabb_buffer = Self::create_buffer(
            Some("AABB buffer"),
            device,
            self.gpu_data.to_bytes().bounding_boxes,
            wgpu::BufferUsages::STORAGE
                | wgpu::BufferUsages::COPY_DST
                | wgpu::BufferUsages::COPY_SRC,
        );

        (object_buffer, aabb_buffer)
    }

    fn pixel_buffer(&self, device: &wgpu::Device) -> wgpu::Buffer {
        let (x, y) = self.image_resolution;
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
        Self::create_buffer(
            Some("Pixels buffer"),
            device,
            self.gpu_data.to_bytes().camera,
            wgpu::BufferUsages::UNIFORM
                | wgpu::BufferUsages::COPY_DST
                | wgpu::BufferUsages::COPY_SRC,
        )
    }

    fn render_settings_buffer(&self, device: &wgpu::Device) -> wgpu::Buffer {
        Self::create_buffer(
            Some("Render options buffer"),
            device,
            self.gpu_data.to_bytes().render,
            wgpu::BufferUsages::UNIFORM
                | wgpu::BufferUsages::COPY_DST
                | wgpu::BufferUsages::COPY_SRC,
        )
    }

    fn random_numbers_buffer(&self, device: &wgpu::Device) -> wgpu::Buffer {
        let (x, y) = self.image_resolution;
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
