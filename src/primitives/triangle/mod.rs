pub mod parser;

use crate::{traits::Intersectable};

use super::{Hit, Normal, Position, Ray};

use glam::Vec3;

/// Single indices of a triangle.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TriangleIndex {
    vertex_indices: (usize, usize, usize),
    normal_index: usize,
    material_index: usize,
}

impl TriangleIndex {
    /// Creates a new triangle from the given vertex indices, normal index and
    /// material index.
    ///
    /// ### Arguments
    /// * `vertex_indices` - The vertex indices of the triangle.
    /// * `normal_index` - The normal index of the triangle.
    /// * `material_index` - The material index of the triangle.
    ///
    /// ### Return value
    /// The newly created triangle.
    #[inline]
    pub fn new(
        vertex_indices: (usize, usize, usize),
        normal_index: usize,
        material_index: usize,
    ) -> Self {
        Self {
            vertex_indices,
            normal_index,
            material_index,
        }
    }

    /// Returns the vertex indices of the triangle.
    ///
    /// ### Return value
    /// (v1, v2, v3)
    #[inline]
    pub fn vertex_indices(&self) -> (usize, usize, usize) {
        self.vertex_indices
    }

    /// Returns the material index of the triangle.
    #[inline]
    pub fn material_index(&self) -> usize {
        self.material_index
    }

    /// Returns the normal index of the triangle.
    #[inline]
    pub fn normal_index(&self) -> usize {
        self.normal_index
    }

    /// Return vertex indices, normal index and material index as a tuple in the
    /// order specified below.
    ///
    /// ### Return value
    /// (v1, v2, v3, n, m)
    #[inline]
    pub fn indices(&self) -> (usize, usize, usize, usize, usize) {
        (
            self.vertex_indices.0,
            self.vertex_indices.1,
            self.vertex_indices.2,
            self.normal_index,
            self.material_index,
        )
    }
}

impl From<&Vec<usize>> for TriangleIndex {
    /// Creates a new triangle from a vec of values. The vec must contain
    /// at least 4 values. If it contains more than 5 values, only the first 5
    /// will be used.
    ///
    /// ### Panics
    /// Panics if the vec is shorter than 4 elements.
    fn from(values: &Vec<usize>) -> Self {
        Self::from(values.as_slice())
    }
}

impl<const N: usize> From<[usize; N]> for TriangleIndex {
    /// Creates a new triangle from a array of values. The array must contain
    /// at least 4 values. If it contains more than 5 values, only the first 5
    /// will be used.
    ///
    /// ### Panics
    /// Panics if the array is shorter than 4 elements.
    fn from(value: [usize; N]) -> Self {
        Self::from(&value[..])
    }
}

impl From<&[usize]> for TriangleIndex {
    /// Creates a new triangle from a slice of values. The slice must contain
    /// at least 4 values. If it contains more than 5 values, only the first 5
    /// will be used.
    ///
    /// ### Panics
    /// Panics if the slice is shorter than 4 elements.
    fn from(values: &[usize]) -> Self {
        if values.len() >= 5 {
            Self::new((values[0], values[1], values[2]), values[3], values[4])
        } else if values.len() == 4 {
            Self::new((values[0], values[1], values[2]), values[3], 0)
        } else {
            panic!("Invalid triangle definition: {:?}", values);
        }
    }
}

impl Default for TriangleIndex {
    /// Creates a new triangle with all indices set to 0.
    fn default() -> Self {
        Self::new((0, 0, 0), 0, 0)
    }
}

/// Struct of all data needed to define a triangle. This includes the vertices,
/// the normal and the material. Data is stored as references.
pub struct Triangle<'mesh> {
    pub vertex_positions: (&'mesh Position, &'mesh Position, &'mesh Position),
    pub normal: &'mesh Normal,
    pub material_index: usize,
}

impl<'mesh> Triangle<'mesh> {
    #[inline]
    pub fn new(
        vertex_positions: (&'mesh Position, &'mesh Position, &'mesh Position),
        normal: &'mesh Normal,
        material_index: usize,
    ) -> Self {
        Self {
            vertex_positions,
            normal,
            material_index,
        }
    }

    /// Returns the minimum vertex positions of the triangle. This is useful
    /// for bounding box calculations.
    #[inline]
    pub fn min(&self) -> Vec3 {
        let (&v0, &v1, &v2) = self.vertex_positions;
        v0.min(v1).min(v2)
    }

    /// Returns the maximum vertex positions of the triangle. This is useful
    /// for bounding box calculations.
    #[inline]
    pub fn max(&self) -> Vec3 {
        let (&v0, &v1, &v2) = self.vertex_positions;
        v0.max(v1).max(v2)
    }
}

impl<'mesh> Intersectable for Triangle<'mesh> {
    /// Intersects a ray with the triangle. [Möller–Trumbore
    /// intersection](https://www.scratchapixel.com/lessons/3d-basic-rendering/ray-tracing-rendering-a-triangle/ray-triangle-intersection-geometric-solution.html)
    ///
    /// ### Arguments
    /// - `ray` - The ray to intersect with.
    /// - `t_min` - The minimum distance to consider.
    /// - `t_max` - The maximum distance to consider.
    ///
    /// ### Return value
    /// The hit data if the ray intersects with the triangle, `None` otherwise.
    fn intersect(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<Hit> {
        let (&v0, &v1, &v2) = self.vertex_positions;
        let edge0 = v1 - v0;
        let edge1 = v2 - v0;
        let h = ray.direction.cross(edge1);
        let a = edge0.dot(h);

        if a.abs() < f32::EPSILON {
            return None;
        }

        let f = 1.0 / a;
        let s = ray.origin - v0;
        let u = f * s.dot(h);

        if !(0.0..=1.0).contains(&u) {
            return None;
        }

        let q = s.cross(edge0);
        let v = f * ray.direction.dot(q);

        if v < 0.0 || u + v > 1.0 {
            return None;
        }

        let t = f * edge1.dot(q);

        if t > t_min && t < t_max {
            return Some(Hit::new(
                ray.at(t),
                t,
                *ray,
                *self.normal,
                self.material_index,
            ));
        }
        None
    }
}

#[cfg(test)]
mod parser_tests;
