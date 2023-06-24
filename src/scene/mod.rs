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
pub use renderer::SceneRenderer;

pub struct Scene<'this> {
    objects: Vec<Object<'this>>,
    triangle_mesh: &'this TriangleMesh,
}

impl<'this> Scene<'this> {
    #[inline]
    pub fn new(triangle_mesh: &'this TriangleMesh, objects: Vec<Object<'this>>) -> Self {
        Self {
            objects,
            triangle_mesh,
        }
    }

    #[inline]
    pub fn get_material(&self, material_index: usize) -> Option<&Material> {
        self.triangle_mesh.get_material(material_index)
    }

    /// Get reference to the triangle mesh.
    #[inline]
    pub fn triangle_mesh(&self) -> &'this TriangleMesh {
        self.triangle_mesh
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