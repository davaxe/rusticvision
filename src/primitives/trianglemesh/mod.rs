use super::material::Material;
use super::{Normal, Position, TexCoord, TriangleIndex, Triangle};

pub mod parser;
#[cfg(test)]
mod parser_tests;

/// Triangle mesh primitive. Stores the vertices, triangles and normals that
/// make up the mesh.
#[derive(Debug, Clone, PartialEq)]
pub struct TriangleMesh {
    vertex_positions: Vec<Position>,
    triangle_normals: Vec<Normal>,
    materials: Vec<Material>,
}

impl TriangleMesh {
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

    pub fn vertex_positions(&self) -> &Vec<Position> {
        &self.vertex_positions
    }

    pub fn triangle_normals(&self) -> &Vec<Normal> {
        &self.triangle_normals
    }

    pub fn materials(&self) -> &Vec<Material> {
        &self.materials
    }

    pub fn get_triangle(&self, triangle_index: &TriangleIndex) -> Triangle {
        let (v1, v2, v3, n, m) = triangle_index.indices();
        let v1_position = self.vertex_positions.get(v1).expect("Invalid vertex index");
        let v2_position = self.vertex_positions.get(v2).expect("Invalid vertex index");
        let v3_position = self.vertex_positions.get(v3).expect("Invalid vertex index");
        let triangle_normal = self.triangle_normals.get(n).expect("Invalid triangle index");
        let triangle_material = self.materials.get(m).expect("Invalid triangle index");
        Triangle::new(
            (v1_position, v2_position, v3_position),
            triangle_normal,
            triangle_material,
        )
    }
}
