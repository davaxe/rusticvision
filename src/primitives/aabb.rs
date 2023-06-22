use super::{Hit, Ray};
use crate::traits::Intersectable;

use glam::Vec3;

#[derive(Debug)]
pub struct BoundingBox {
    min: Vec3,
    max: Vec3,
}

impl BoundingBox {
    #[inline]
    pub fn new(min: Vec3, max: Vec3) -> Self {
        Self { min, max }
    }

    /// Check if a ray intersects with the bounding box.
    ///
    /// [Reference](https://medium.com/@bromanz/another-view-on-the-classic-ray-aabb-intersection-algorithm-for-bvh-traversal-41125138b525).
    #[inline]
    pub fn intersect(&self, ray: &Ray, t_min: f32, t_max: f32) -> bool {
        let inv_dir = ray.direction.recip();
        let t0 = (self.min - ray.origin) * inv_dir;
        let t1 = (self.max - ray.origin) * inv_dir;

        let t_small = t0.min(t1);
        let t_big = t0.max(t1);

        let t_min = t_min.max(t_small.max_element());
        let t_max = t_max.min(t_big.min_element());

        t_max >= t_min
    }
}
