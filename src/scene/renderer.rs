use std::sync::Mutex;

use crate::traits::Intersectable;

use super::{Camera, Scene};

use glam::Vec3;
use image::RgbImage;
use itertools::Itertools;
use rayon::prelude::*;

pub struct SceneRenderer<'scene> {
    camera: &'scene Camera,
    scene: &'scene Scene<'scene>,
    sample_count: u32,
    recursion_depth: u32,
}

impl<'scene> SceneRenderer<'scene> {
    #[inline]
    pub fn new(camera: &'scene Camera, scene: &'scene Scene<'scene>) -> Self {
        Self {
            camera,
            scene,
            sample_count: 1,
            recursion_depth: 1,
        }
    }

    #[inline]
    pub fn set_sample_count(&mut self, sample_count: u32) {
        self.sample_count = sample_count;
    }

    #[inline]
    pub fn set_recursion_depth(&mut self, recursion_depth: u32) {
        self.recursion_depth = recursion_depth;
    }

    pub fn render(&self) -> RgbImage {
        let (width, height) = self.camera.get_dimensions();
        let image: Mutex<image::RgbImage> = Mutex::new(RgbImage::new(width, height));
        (0..width)
            .cartesian_product(0..height)
            .par_bridge()
            .for_each(|(x, y)| {
                let pixel = self.render_pixel(x, y);
                let mut image = image.lock().unwrap();
                image.put_pixel(x, y, Self::vec3_to_rgb(pixel));
            });
        image.into_inner().unwrap()
    }

    #[inline]
    fn render_pixel(&self, x: u32, y: u32) -> Vec3 {
        let ray = self.camera.get_ray(x, y);
        match self.scene.intersect(&ray, 0.001, 100.0) {
            Some(hit) => {
                let _material = self.scene.get_material(hit.material_index).unwrap();
                hit.normal
            }
            None => Vec3::new(0.0, 0.0, 0.0),
        }
    }

    #[inline]
    fn vec3_to_rgb(color: Vec3) -> image::Rgb<u8> {
        let r = (color.x * 255.0) as u8;
        let g = (color.y * 255.0) as u8;
        let b = (color.z * 255.0) as u8;
        image::Rgb([r, g, b])
    }
}