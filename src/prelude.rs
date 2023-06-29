

use crate::{
    parser,
    scene::{camera::CameraBuilder, Camera, Scene, SceneRenderer},
};

use glam::Vec3A;
use image::RgbImage;

pub struct RayTracer {
    camera_builder: CameraBuilder,
    directory: Option<String>,
    obj_file: Option<String>,
    camera: Option<Camera>,
    sample_count: Option<u32>,
    recursion_depth: Option<u32>,
}

impl RayTracer {
    #[inline]
    pub fn new() -> Self {
        Self {
            camera_builder: CameraBuilder::new(),
            directory: None,
            obj_file: None,
            camera: None,
            sample_count: None,
            recursion_depth: None,
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

    /// Sets the resolution of the image to be rendered.
    #[inline]
    pub fn resolution(mut self, width: u32, height: u32) -> Self {
        self.camera_builder = self.camera_builder.with_width(width).with_height(height);
        self
    }

    /// Sets camera position.
    #[inline]
    pub fn camera_position(mut self, x: f32, y: f32, z: f32) -> Self {
        self.camera_builder = self.camera_builder.with_position(Vec3A::new(x, y, z));
        self
    }

    /// Sets camera target, ie the point the camera is looking at. Defaults to
    /// origin (0, 0, 0).
    #[inline]
    pub fn camera_target(mut self, x: f32, y: f32, z: f32) -> Self {
        self.camera_builder = self.camera_builder.with_target(Vec3A::new(x, y, z));
        self
    }

    /// Set the camera to be used for rendering, overriding any camera set with
    /// `camera_position` and `camera_target`.
    ///
    /// Use this if you want a more customized camera.
    #[inline]
    pub fn camera(mut self, camera: Camera) -> Self {
        self.camera = Some(camera);
        self
    }

    /// Sets the number of samples to take per pixel.
    #[inline]
    pub fn sample_count(mut self, sample_count: u32) -> Self {
        self.sample_count = Some(sample_count);
        self
    }

    /// Sets the recursion depth for the ray tracing.
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

        let built_camera = self.camera_builder.clone().build();

        let camera = match &self.camera {
            Some(camera) => camera,
            None => &built_camera,
        };

        let (mesh, mat_map, obj_map) = parser::get_triangle_mesh_and_obj_map(directory, obj_file);

        let (objects, mesh) = parser::get_objects(mesh, &obj_map, &mat_map);

        let scene = Scene::new(mesh, objects);
        let mut renderer = SceneRenderer::new(camera, &scene);

        renderer.set_sample_count(self.sample_count.unwrap_or(1));
        renderer.set_recursion_depth(self.recursion_depth.unwrap_or(1));

        renderer.render()
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
