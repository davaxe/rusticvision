use std::sync::Mutex;

use crate::traits::Intersectable;

use super::{camera::CameraBuilder, Camera, Scene};

use glam::Vec3;
use image::RgbImage;
use itertools::Itertools;
use rayon::prelude::*;

pub struct SceneRenderer<'scene> {
    camera: Camera,
    scene: &'scene Scene<'scene>,
    sample_count: u32,
    recursion_depth: u32,
}

impl<'scene> SceneRenderer<'scene> {
    pub fn new(camera: Camera, scene: &'scene Scene<'scene>) -> Self {
        Self {
            camera,
            scene,
            sample_count: 1,
            recursion_depth: 1,
        }
    }

    pub fn render(&self) {
        let (width, height) = self.camera.get_dimensions();
        let image: Mutex<image::RgbImage> = Mutex::new(RgbImage::new(width, height));
        (0..width)
            .cartesian_product(0..height)
            .par_bridge()
            .for_each(|(x, y)| {
                let pixel = self.render_pixel(x, y);
                let mut image = image.lock().unwrap();
                image.put_pixel(x, y, Self::Vec3_to_rgb(pixel));
            });
        let image = image.into_inner().unwrap();
        image.save("output.png").unwrap();
    }

    fn render_pixel(&self, x: u32, y: u32) -> Vec3 {
        let ray = self.camera.get_ray(x, y);
        match self.scene.intersect(&ray, 0.001, 100.0) {
            Some(hit) => {
                let material = self.scene.get_material(hit.material_index).unwrap();
                hit.normal
            }
            None => Vec3::new(0.0, 0.0, 0.0),
        }
    }

    fn Vec3_to_rgb(color: Vec3) -> image::Rgb<u8> {
        let r = (color.x * 255.0) as u8;
        let g = (color.y * 255.0) as u8;
        let b = (color.z * 255.0) as u8;
        image::Rgb([r, g, b])
    }
}

pub fn render(directory: &str, obj_file: &str) {
    let (mesh, material_map, obj_map) =
        super::parser::get_triangle_mesh_and_obj_map(directory, obj_file);

    let objects = super::parser::get_objects(&mesh, &obj_map, &material_map);

    let camera = CameraBuilder::new()
        .with_position(Vec3::new(0.0, 0.0,  -5.0))
        .with_direction(Vec3::new(0.0, 0.0, 5.0).normalize())
        .with_vertical_fov(39.6)
        .build();
    let scene = Scene::new(&mesh, objects);
    let scene_renderer = SceneRenderer::new(camera, &scene);
    scene_renderer.render();
}
