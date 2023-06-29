use std::io::Write;
use std::sync::{atomic::AtomicU32, Mutex};

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
