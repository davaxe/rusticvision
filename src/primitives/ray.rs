use glam::Vec3;

use super::{Normal, Position};

/// A light ray in 3D space. The ray is defined by an origin and a direction.
/// The directions is not necessarily normalized.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Ray {
    pub origin: Position,
    pub direction: Vec3,
}

impl Ray {
    /// Creates a new ray from the given origin and direction.
    #[inline]
    pub fn new(origin: Vec3, direction: Vec3) -> Self {
        Self { origin, direction }
    }

    /// Returns the position of the ray at the given t value.
    #[inline]
    pub fn at(&self, t: f32) -> Vec3 {
        self.origin + self.direction * t
    }

    /// Returns the distance of the ray at the given `t` value. If direction is
    /// normalized the distance is equal to `t`.
    #[inline]
    pub fn at_dist(&self, t: f32) -> f32 {
        self.at_dist_squared(t).sqrt()
    }

    /// Returns the squared distance of the ray at the given t value.
    #[inline]
    pub fn at_dist_squared(&self, t: f32) -> f32 {
        (self.direction * t).length_squared()
    }

    /// Returns true if the ray is the default ray (origin and direction are
    /// zero vectors).
    #[inline]
    pub fn is_default(&self) -> bool {
        self.direction == Vec3::ZERO
    }
}

impl Default for Ray {
    /// Returns the default ray. The origin and direction are zero vectors.
    /// A direction of zero length is not a valid direction.
    fn default() -> Self {
        Self {
            origin: Vec3::ZERO,
            direction: Vec3::ZERO,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Hit {
    pub hit_point: Position,
    pub normal: Normal,
    pub distance: f32,
    pub incoming: Ray,
    pub material_index: usize,
}

impl Hit {
    #[inline]
    pub fn new(
        hit_point: Position,
        distance: f32,
        incoming: Ray,
        normal: Normal,
        material_index: usize,
    ) -> Self {
        Self {
            hit_point,
            normal,
            distance,
            incoming,
            material_index,
        }
    }

    pub fn closest_hit(self, other: Self) -> Self {
        if self.distance < other.distance {
            self
        } else {
            other
        }
    }
}
