use std::sync::Arc;

use crate::primitives::{BoundingBox, Hit, Ray, Triangle, TriangleIndex, TriangleMesh};

use crate::traits::Intersectable;

use glam::Vec3A;

/// Struct representing an object in the scene. An object is essentially a collections
/// of triangles with a bounding box.
#[derive(Debug)]
pub struct Object {
    pub identifier: String,
    triangle_start_index: usize,
    triangle_count: usize,
    mesh: Arc<TriangleMesh>,
    bounding_box: BoundingBox,
}

impl Object {
    pub fn new(
        identifier: String,
        triangle_start_index: usize,
        triangle_count: usize,
        mesh: Arc<TriangleMesh>,
    ) -> Self {
        let bounding_box = Self::bounding_box(triangle_start_index, triangle_count, mesh.as_ref());
        Self {
            identifier,
            mesh,
            triangle_start_index,
            triangle_count,
            bounding_box,
        }
    }

    /// Get the bounding box of the object by iterating over the triangles of the object
    /// and taking the minimum and maximum coordinates.
    #[inline]
    fn bounding_box(
        triangle_start_index: usize,
        triangle_count: usize,
        mesh: &TriangleMesh,
    ) -> BoundingBox {
        let triangles =
            &mesh.triangle_indices()[triangle_start_index..triangle_start_index + triangle_count];
        let mut min_coordinates = Vec3A::splat(f32::INFINITY);
        let mut max_coordinates = Vec3A::splat(f32::NEG_INFINITY);
        triangles.iter().for_each(|t_idx| {
            let tri = mesh.get_triangle(t_idx);
            min_coordinates = min_coordinates.min(tri.min());
            max_coordinates = max_coordinates.max(tri.max());
        });
        BoundingBox::new(min_coordinates, max_coordinates)
    }

    /// Convenience function to get the triangles indices of the object.
    #[inline]
    fn triangle_indices(&self) -> &[TriangleIndex] {
        &self.mesh.triangle_indices()
            [self.triangle_start_index..self.triangle_start_index + self.triangle_count]
    }

    /// Convenience function to get iterator over the triangles of the object.
    #[inline]
    fn triangles(&self) -> impl Iterator<Item = Triangle> {
        self.triangle_indices()
            .iter()
            .map(|t_idx| self.mesh.get_triangle(t_idx))
    }
}

impl Intersectable for Object {
    fn intersect(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<Hit> {
        if !self.bounding_box.intersect(ray, t_min, t_max) {
            return None;
        }

        let mut closest_hit: Option<Hit> = None;
        for triangle in self.triangles() {
            if let Some(hit) = triangle.intersect(ray, t_min, t_max) {
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
