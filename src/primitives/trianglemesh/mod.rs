use crate::{material::Material, traits::Intersectable};

use super::{Hit, Normal, Position, Ray, Triangle, TriangleIndex};

/// Triangle mesh primitive. Stores the vertices, triangles and normals that
/// make up the mesh. Also stores all possible materials that can be applied
/// to a triangle.
#[derive(Debug, Clone, PartialEq)]
pub struct TriangleMesh {
    vertex_positions: Vec<Position>,
    triangle_normals: Vec<Normal>,
    materials: Vec<Material>,
    triangle_indices: Vec<TriangleIndex>,
}

impl TriangleMesh {
    /// Create a new triangle mesh from the vertex positions, triangle normals
    /// and materials.
    pub fn new(
        vertex_positions: Vec<Position>,
        triangle_normals: Vec<Normal>,
        triangle_materials: Vec<Material>,
        triangle_indices: Vec<TriangleIndex>,
    ) -> Self {
        Self {
            vertex_positions,
            triangle_normals,
            materials: triangle_materials,
            triangle_indices,
        }
    }

    /// Get reference to the vertex positions.
    #[inline]
    pub fn vertex_positions(&self) -> &[Position] {
        &self.vertex_positions
    }

    #[inline]
    pub fn extend_triangle_indices(&mut self, triangle_indices: &[TriangleIndex]) {
        self.triangle_indices.extend(triangle_indices);
    }

    /// Get reference to the triangle normals.
    #[inline]
    pub fn triangle_normals(&self) -> &[Normal] {
        &self.triangle_normals
    }

    /// Get reference to the materials.
    #[inline]
    pub fn materials(&self) -> &[Material] {
        &self.materials
    }

    #[inline]
    pub fn triangle_indices(&self) -> &[TriangleIndex]{
        &self.triangle_indices
    }

    /// Get material from material index.
    pub fn get_material(&self, material_index: usize) -> Option<&Material> {
        self.materials.get(material_index)
    }


    /// Get triangle from triangle index.
    ///
    /// ### Arguments
    /// - `triangle_index` - The triangle index.
    ///
    /// ### Returns
    /// The triangle containing the vertex positions, normal and material of the
    /// triangle.
    ///
    /// ### Panics
    /// Panics if any of the indices in the `TriangleIndex` are out of invalid.
    pub fn get_triangle(&self, triangle_index: &TriangleIndex) -> Triangle {
        let (v1, v2, v3, n, _) = triangle_index.indices();
        Triangle::new(
            (
                self.vertex_positions.get(v1).expect("Invalid vertex index"),
                self.vertex_positions.get(v2).expect("Invalid vertex index"),
                self.vertex_positions.get(v3).expect("Invalid vertex index"),
            ),
            self.triangle_normals
                .get(n)
                .expect("Invalid triangle index"),
            *triangle_index,
        )
    }

    pub fn get_triangle_from_index(&self, triangle_index: usize) -> Triangle {
        let triangle_index = self.triangle_indices.get(triangle_index).unwrap();
        self.get_triangle(triangle_index)
    }

    /// Test if the given ray intersects the given triangle in the mesh.
    ///
    /// ### Arguments
    /// - `ray` - The ray to test intersection with.
    /// - `triangle_index` - The triangle index of the triangle to test
    /// - `t_min` - The minimum distance along the ray to test for intersection.
    /// - `t_max` - The maximum distance along the ray to test for intersection.
    ///
    /// ### Returns
    /// `Some(Hit)` if the ray intersects the triangle, `None` otherwise.
    ///
    /// ### Panics
    /// Panics if any of the indices in the `TriangleIndex` are invalid.
    pub fn intersect_triangle(
        &self,
        ray: &Ray,
        triangle_index: &TriangleIndex,
        t_min: f32,
        t_max: f32,
    ) -> Option<Hit> {
        let triangle = self.get_triangle(triangle_index);
        triangle.intersect(ray, t_min, t_max)
    }
}
