use crate::primitives::{TriangleMesh, TriangleIndex};

/// Struct representing an object in the scene. An object is essentially a collections
/// of triangles.
pub struct Object<'global> {
    pub identifier: String,
    pub triangles: Vec<TriangleIndex>,
    mesh: &'global TriangleMesh,
}

impl<'global> Object<'global> {
    pub fn new(identifier: String, triangles: Vec<TriangleIndex>, mesh: &'global TriangleMesh) -> Self {
        Self {
            identifier,
            mesh,
            triangles,
        }
    }
}