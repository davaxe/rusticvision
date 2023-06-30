use super::Ray;

use glam::{Vec3, Vec3A};

#[derive(Debug)]
pub struct BoundingBox {
    min: Vec3A,
    max: Vec3A,
}

impl BoundingBox {
    #[inline]
    pub fn new(min: Vec3A, max: Vec3A) -> Self {
        Self { min, max }
    }

    /// Check if a ray intersects with the bounding box.
    ///
    /// [Reference](https://medium.com/@bromanz/another-view-on-the-classic-ray-aabb-intersection-algorithm-for-bvh-traversal-41125138b525).
    ///
    /// ### Arguments
    /// - `ray` - The ray to check for intersection.
    /// - `t_min` - The minimum distance to check for intersection.
    /// - `t_max` - The maximum distance to check for intersection.
    ///
    /// ### Returns
    /// - `true` if the ray intersects with the bounding box.
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

    pub fn min(&self) -> Vec3 {
        self.min.into()
    }

    pub fn max(&self) -> Vec3 {
        self.max.into()
    }

    pub fn bounds(&self) -> (Vec3, Vec3) {
        (self.min.into(), self.max.into())
    }
}
