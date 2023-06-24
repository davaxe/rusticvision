use glam::Vec3A;

use crate::material::Material;

use super::{Normal, Position, TriangleIndex, TriangleMesh};

/// A light ray in 3D space. The ray is defined by an origin and a direction.
/// The directions is not necessarily normalized.
#[derive(Debug, Clone, Copy)]
pub struct Ray {
    pub origin: Position,
    pub direction: Vec3A,
}

impl Ray {
    /// Creates a new ray from the given origin and direction.
    #[inline]
    pub fn new(origin: Vec3A, direction: Vec3A) -> Self {
        Self { origin, direction }
    }

    /// Returns the position of the ray at the given t value.
    #[inline]
    pub fn at(&self, t: f32) -> Vec3A {
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
        self.direction == Vec3A::ZERO
    }
}

impl Default for Ray {
    /// Returns the default ray. The origin and direction are zero vectors.
    /// A direction of zero length is not a valid direction.
    fn default() -> Self {
        Self {
            origin: Vec3A::ZERO,
            direction: Vec3A::ZERO,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Hit {
    pub hit_point: Position,
    pub distance: f32,
    pub incoming: Ray,
    pub triangle_index: TriangleIndex,
}

impl Hit {
    #[inline]
    pub fn new(
        hit_point: Position,
        distance: f32,
        incoming: Ray,
        triangle_index: TriangleIndex,
    ) -> Self {
        Self {
            hit_point,
            distance,
            incoming,
            triangle_index,
        }
    }

    /// Compare two hits and return the one with the smallest distance.
    #[inline]
    pub fn closest_hit(self, other: Self) -> Self {
        if self.distance < other.distance {
            self
        } else {
            other
        }
    }

    /// Returns the normal index of the triangle that was hit.
    ///
    /// Use this to get the normal of the triangle from the mesh.
    #[inline]
    pub fn normal_index(&self) -> usize {
        self.triangle_index.normal_index()
    }

    /// Returns the normal of the triangle that was hit, using the mesh.
    ///
    /// Mesh is needed because the `hit` struct only contains the index of the
    /// normal.
    #[inline]
    pub fn normal(&self, mesh: &TriangleMesh) -> Normal {
        mesh.triangle_normals()[self.normal_index()]
    }

    /// Returns the material index of the triangle that was hit.
    ///
    /// Use this to get the material of the triangle from the mesh.
    #[inline]
    pub fn material_index(&self) -> usize {
        self.triangle_index.material_index()
    }

    /// Returns the material of the triangle that was hit, using the mesh.
    ///
    /// Mesh is needed because the `hit` struct only contains the index of the
    /// material.
    #[inline]
    pub fn material(&self, mesh: &TriangleMesh) -> Material {
        mesh.materials()[self.material_index()]
    }

    /// Returns a random outgoing ray from the hit point.
    #[inline]
    pub fn random_outgoing_ray(&self, mesh: &TriangleMesh) -> Ray {
        let triangle = mesh.get_triangle(&self.triangle_index);

        // Local coordinate system
        let up = *triangle.normal;
        let right = (*triangle.vertex_positions.1 - *triangle.vertex_positions.0).normalize();
        let forward = right.cross(up);

        // Random spherical coordinates (theta, phi) to get a random direction
        let theta = rand::random::<f32>() * 2.0 * std::f32::consts::PI;
        let phi = rand::random::<f32>() * std::f32::consts::PI;

        let spherical_point =
            Vec3A::new(phi.sin() * theta.cos(), phi.sin() * theta.sin(), phi.cos());

        let direction =
            right * spherical_point.x + forward * spherical_point.y + up * spherical_point.z;
        let direction = direction.normalize();

        Ray::new(self.hit_point, direction)
    }
}
