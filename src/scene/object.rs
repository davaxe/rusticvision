use crate::primitives::{BoundingBox, Hit, Ray, TriangleIndex, TriangleMesh};

use crate::traits::Intersectable;

use glam::Vec3A;

/// Struct representing an object in the scene. An object is essentially a collections
/// of triangles.
#[derive(Debug)]
pub struct Object<'mesh> {
    pub identifier: String,
    pub triangles: Vec<TriangleIndex>,
    mesh: &'mesh TriangleMesh,
    bounding_box: BoundingBox,
}

impl<'mesh> Object<'mesh> {
    pub fn new(
        identifier: String,
        triangles: Vec<TriangleIndex>,
        mesh: &'mesh TriangleMesh,
    ) -> Self {
        let bounding_box = Self::bounding_box(&triangles, mesh);
        Self {
            identifier,
            mesh,
            triangles,
            bounding_box,
        }
    }

    fn bounding_box(triangles: &[TriangleIndex], mesh: &TriangleMesh) -> BoundingBox {
        let mut min_coordinates = Vec3A::splat(f32::INFINITY);
        let mut max_coordinates = Vec3A::splat(f32::NEG_INFINITY);
        triangles.iter().for_each(|t_idx| {
            let tri = mesh.get_triangle(t_idx);
            min_coordinates = min_coordinates.min(tri.min());
            max_coordinates = max_coordinates.max(tri.max());
        });
        BoundingBox::new(min_coordinates, max_coordinates)
    }
}

impl<'mesh> Intersectable for Object<'mesh> {
    fn intersect(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<Hit> {
        if !self.bounding_box.intersect(ray, t_min, t_max) {
            return None;
        }

        let mut closest_hit: Option<Hit> = None;
        for t_idx in &self.triangles {
            if let Some(hit) = self.mesh.intersect_triangle(ray, t_idx, t_min, t_max) {
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
