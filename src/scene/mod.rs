pub mod camera;
pub mod object;
pub mod parser;
pub mod renderer;

use crate::{
    material::Material,
    primitives::{Hit, Ray, TriangleMesh},
    traits::Intersectable,
};

use object::Object;

pub use camera::Camera;

pub struct Scene<'this> {
    objects: Vec<Object<'this>>,
    triangle_mesh: &'this TriangleMesh,
}

impl<'this> Scene<'this> {
    pub fn new(triangle_mesh: &'this TriangleMesh, objects: Vec<Object<'this>>) -> Self {
        Self {
            objects,
            triangle_mesh,
        }
    }

    pub fn get_material(&self, material_index: usize) -> Option<&Material> {
        self.triangle_mesh.get_material(material_index)
    }

    pub fn debug_print_objects(&self) {
        for object in &self.objects {
            println!("Object: {}", object.identifier);
            for triangle in &object.triangles {
                println!("\tTriangle: {:?}", triangle);
            }
            println!();
        }
    }
}

impl<'this> Intersectable for Scene<'this> {
    fn intersect(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<Hit> {
        let mut closest_hit: Option<Hit> = None;
        for object in &self.objects {
            if let Some(hit) = object.intersect(ray, t_min, t_max) {
                if let Some(closest) = closest_hit {
                    closest_hit = Some(hit.closest_hit(closest));
                } else {
                    closest_hit = Some(hit);
                }
            }
        }
        closest_hit
    }
}
