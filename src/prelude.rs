use image::RgbImage;

use crate::{data_structures::*, parser::parse_obj, renderer::Renderer};

pub struct RayTracer {
    camera_builder: CameraBuilder,
    directory: Option<String>,
    obj_file: Option<String>,
    sample_count: Option<u32>,
    recursion_depth: Option<u32>,
    image_resolution: Option<(u32, u32)>,
}

impl RayTracer {
    #[inline]
    pub fn new() -> Self {
        Self {
            camera_builder: CameraBuilder::new(),
            directory: None,
            obj_file: None,
            sample_count: None,
            recursion_depth: None,
            image_resolution: None,
        }
    }

    /// Sets the directory of the obj file to be loaded.
    ///
    /// Needs to be set before calling `render` or `render_save`.
    #[inline]
    pub fn directory(mut self, directory: &str) -> Self {
        self.directory = Some(directory.to_string());
        self
    }

    /// Sets the obj file to be loaded.
    ///
    /// Needs to be set before calling `render` or `render_save`.
    #[inline]
    pub fn obj_file(mut self, obj_file: &str) -> Self {
        self.obj_file = Some(obj_file.to_string());
        self
    }

    /// Sets the directory and obj file to be loaded.
    ///
    /// Directory and obj file must be specified before calling `render` or
    /// `render_save`.  This is a convenience method for calling `directory` and
    /// `obj_file` in succession.
    #[inline]
    pub fn file(mut self, directory: &str, obj_file: &str) -> Self {
        self.directory = Some(directory.to_string());
        self.obj_file = Some(obj_file.to_string());
        self
    }

    /// Sets the resolution of the image to be rendered. Defaults to (800, 600).
    #[inline]
    pub fn resolution(mut self, width: u32, height: u32) -> Self {
        self.image_resolution = Some((width, height));
        self
    }

    /// Sets camera position. Defaults to (0, 1, 0).
    #[inline]
    pub fn camera_position(mut self, x: f32, y: f32, z: f32) -> Self {
        self.camera_builder = self.camera_builder.with_position(x, y, z);
        self
    }

    /// Sets camera target, ie the point the camera is looking at. Defaults to
    /// origin (0, 0, 0).
    #[inline]
    pub fn camera_target(mut self, x: f32, y: f32, z: f32) -> Self {
        self.camera_builder = self.camera_builder.with_target(x, y, z);
        self
    }

    /// Sets the camera's vertical field of view in degrees. Defaults to 39.6,
    /// which is the default blender value.
    #[inline]
    pub fn camera_vertical_fov(mut self, fov: f32) -> Self {
        self.camera_builder = self.camera_builder.with_vertical_fov(fov);
        self
    }

    /// Set the camera's clipping planes. Defaults to (0.1, 1000.0).
    #[inline]
    pub fn camera_clipping(mut self, near: f32, far: f32) -> Self {
        self.camera_builder = self.camera_builder.with_z_far(far).with_z_near(near);
        self
    }

    /// Sets the number of samples to take per pixel. Defaults to 1.
    #[inline]
    pub fn sample_count(mut self, sample_count: u32) -> Self {
        self.sample_count = Some(sample_count);
        self
    }

    /// Sets the recursion depth for the ray tracing. Defaults to 1.
    #[inline]
    pub fn recursion_depth(mut self, recursion_depth: u32) -> Self {
        self.recursion_depth = Some(recursion_depth);
        self
    }

    /// Renders the scene and returns the image.
    ///
    /// ### Panics
    /// If the directory or obj file have not been specified.
    pub fn render(&self) -> RgbImage {
        // Directory and obj file must be specified
        if self.directory.is_none() || self.obj_file.is_none() {
            panic!("Directory and obj file must be specified");
        }

        let directory = self.directory.as_ref().unwrap();
        let obj_file = self.obj_file.as_ref().unwrap();

        let obj_data = parse_obj(directory, obj_file);

        // Get the image resolution, sample count, and recursion depth
        let (width, height) = self.image_resolution.unwrap_or((800, 600));
        let samples = self.sample_count.unwrap_or(1);
        let bounces = self.recursion_depth.unwrap_or(1);

        // Create GPU data and load it with the obj data
        let mut gpu_data = GPUData::new();
        gpu_data.set_bounding_boxes(obj_data.bounding_boxes);
        gpu_data.set_triangles(obj_data.triangle_data);
        gpu_data.set_materials(obj_data.material_data);
        gpu_data.set_vertex_positions(obj_data.vertex_data.positions);
        gpu_data.set_vertex_normals(obj_data.vertex_data.normals);
        gpu_data.set_objects(obj_data.object_data);

        gpu_data.set_camera(self.camera_builder.clone().build((width, height)));

        gpu_data.set_render(RenderData::new(width, height, samples, bounces));

        let renderer = Renderer::new(gpu_data, (width, height));
        pollster::block_on(renderer.render()).unwrap()
    }

    /// Renders the scene and saves the image to the specified file path.
    ///
    /// ### Panics
    /// If the directory or obj file have not been specified.
    pub fn render_save(&self, file_path: &str) {
        let image = self.render();
        image.save(file_path).unwrap();
    }
}

impl Default for RayTracer {
    fn default() -> Self {
        Self::new()
    }
}

/// Private struct for easily building a camera.
#[derive(Clone, Debug)]
struct CameraBuilder {
    position: Option<[f32; 3]>,
    target: Option<[f32; 3]>,
    z_near: Option<f32>,
    z_far: Option<f32>,
    vertical_fov: Option<f32>,
}

impl CameraBuilder {
    pub fn new() -> Self {
        Self {
            position: None,
            target: None,
            z_near: None,
            z_far: None,
            vertical_fov: None,
        }
    }

    /// The position of the camera. Defaults to (0, 0, 0)
    #[inline]
    pub fn with_position(mut self, x: f32, y: f32, z: f32) -> Self {
        self.position = Some([x, y, z]);
        self
    }

    /// The point the camera is looking at. Defaults to (0, 0, 1)
    #[inline]
    pub fn with_target(mut self, x: f32, y: f32, z: f32) -> Self {
        self.target = Some([x, y, z]);
        self
    }

    // Near clipping plane distance. Anything closer than this will not be
    // rendered.
    #[inline]
    pub fn with_z_near(mut self, z_near: f32) -> Self {
        self.z_near = Some(z_near);
        self
    }

    /// Far clipping plane distance. Anything beyond this will not be rendered.
    #[inline]
    pub fn with_z_far(mut self, z_far: f32) -> Self {
        self.z_far = Some(z_far);
        self
    }

    /// Vertical field of view in degrees
    #[inline]
    pub fn with_vertical_fov(mut self, vertical_fov: f32) -> Self {
        self.vertical_fov = Some(vertical_fov);
        self
    }

    /// Build the camera, based on the parameters set.
    #[inline]
    pub fn build(self, image_size: (u32, u32)) -> CameraData {
        let vertical_fov = self.vertical_fov.unwrap_or(39.6); // blender default
        let aspect_ratio = image_size.0 as f32 / image_size.1 as f32;
        let z_near = self.z_near.unwrap_or(0.1);
        let z_far = self.z_far.unwrap_or(1000.0);
        let position = self.position.unwrap_or([0.0, 1.0, 0.0]);
        let target = self.target.unwrap_or([0.0, 0.0, 0.0]);

        let projection =
            glam::Mat4::perspective_rh(vertical_fov.to_radians(), aspect_ratio, z_near, z_far);
        let inverse_projection = projection.inverse();
        let view = glam::Mat4::look_at_rh(position.into(), target.into(), glam::Vec3::Y);
        let inverse_view = view.inverse();

        CameraData::new(
            inverse_projection.to_cols_array(),
            inverse_view.to_cols_array(),
            position,
        )
    }
}

impl Default for CameraBuilder {
    fn default() -> Self {
        Self::new()
    }
}
