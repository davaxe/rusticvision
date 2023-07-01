pub mod camera;
pub mod object;
pub mod renderer;

use std::sync::Arc;

use crate::{
    material::Material,
    primitives::{Hit, Ray, TriangleMesh},
    traits::Intersectable,
};

use itertools::Itertools;
use object::Object;

pub use camera::Camera;
pub use renderer::SceneRenderer;

pub struct Scene {
    objects: Vec<Object>,
    triangle_mesh: Arc<TriangleMesh>,
}

impl Scene {
    #[inline]
    pub fn new(triangle_mesh: Arc<TriangleMesh>, objects: Vec<Object>) -> Self {
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
    pub fn triangle_mesh(&self) -> &TriangleMesh {
        self.triangle_mesh.as_ref()
    }

    pub fn gpu_vertex_pos_data(&self) -> Vec<[f32; 4]> {
        self.triangle_mesh
            .vertex_positions()
            .iter()
            .map(|&v| [v.x, v.y, v.z, 0.0])
            .collect()
    }

    pub fn gpu_triangle_normal_data(&self) -> Vec<[f32; 4]> {
        self.triangle_mesh
            .triangle_normals()
            .iter()
            .map(|&v| [v.x, v.y, v.z, 0.0])
            .collect()
    }

    pub fn gpu_triangle_index_data(&self) -> Vec<[u32; 5]> {
        self.triangle_mesh
            .triangle_indices()
            .iter()
            .map(|t_idx| {
                let (v1, v2, v3, n, m) = t_idx.indices();
                [v1 as u32, v2 as u32, v3 as u32, n as u32, m as u32]
            })
            .collect_vec()
    }

    pub fn gpu_material_data(&self) -> Vec<[f32; 8]> {
        self.triangle_mesh
            .materials()
            .iter()
            .map(|mat| {
                let d = mat.diffuse_color;
                let e = mat.emissive_color;
                [d.x, d.y, d.z, 0f32, e.x, e.y, e.z, 0f32]
            })
            .collect_vec()
    }
}

impl Intersectable for Scene {
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
