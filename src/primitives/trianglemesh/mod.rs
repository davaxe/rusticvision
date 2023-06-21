use super::material::Material;
use super::{Normal, Position, TexCoord, Triangle, TriangleIndex};

pub mod parser;

/// Triangle mesh primitive. Stores the vertices, triangles and normals that
/// make up the mesh. Also stores all possible materials that can be applied
/// to a triangle.
#[derive(Debug, Clone, PartialEq)]
pub struct TriangleMesh {
    vertex_positions: Vec<Position>,
    triangle_normals: Vec<Normal>,
    materials: Vec<Material>,
}

impl TriangleMesh {
    /// Create a new triangle mesh from the vertex positions, triangle normals
    /// and materials.
    pub fn new(
        vertex_positions: Vec<Position>,
        triangle_normals: Vec<Normal>,
        triangle_materials: Vec<Material>,
    ) -> Self {
        Self {
            vertex_positions,
            triangle_normals,
            materials: triangle_materials,
        }
    }

    /// Get reference to the vertex positions.
    pub fn vertex_positions(&self) -> &Vec<Position> {
        &self.vertex_positions
    }

    /// Get reference to the triangle normals.
    pub fn triangle_normals(&self) -> &Vec<Normal> {
        &self.triangle_normals
    }

    /// Get reference to the materials.
    pub fn materials(&self) -> &Vec<Material> {
        &self.materials
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
        let (v1, v2, v3, n, m) = triangle_index.indices();
        Triangle::new(
            (
                self.vertex_positions.get(v1).expect("Invalid vertex index"),
                self.vertex_positions.get(v2).expect("Invalid vertex index"),
                self.vertex_positions.get(v3).expect("Invalid vertex index"),
            ),
            self.triangle_normals
                .get(n)
                .expect("Invalid triangle index"),
            self.materials.get(m).expect("Invalid triangle index"),
        )
    }
}

#[cfg(test)]
mod parser_tests;
