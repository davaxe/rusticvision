use super::primitives::{Hit, Ray};

pub trait Intersectable {
    /// Returns the closest intersection between the ray and the object.
    ///
    /// # Arguments
    /// - `ray` - The ray to check for intersection.
    /// - `t_min` - The minimum distance along the ray to check for
    ///   intersection.
    /// - `t_max` - The maximum distance along the ray to check for
    ///   intersection.
    ///
    /// # Returns
    /// The closest intersection (Hit) between the ray and the object. None if
    /// there is no intersection.
    fn intersect(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<Hit>;

    /// Default implementations is same as intersect() method with ray from
    /// t_min to infinity
    fn intersect_inf(&self, ray: &Ray, t_min: f32) -> Option<Hit> {
        self.intersect(ray, t_min, f32::INFINITY)
    }

    /// Returns true if the ray intersects the object.
    ///
    /// Default implementation is to check if intersect() returns Some.
    ///
    /// # Arguments
    /// - `ray` - The ray to check for intersection.
    /// - `t_min` - The minimum distance along the ray to check for intersection.
    /// - `t_max` - The maximum distance along the ray to check for intersection.
    ///
    /// # Returns
    /// True if the ray intersects the object.
    fn intersected(&self, ray: &Ray, t_min: f32, t_max: f32) -> bool {
        self.intersect(ray, t_min, t_max).is_some()
    }
}
