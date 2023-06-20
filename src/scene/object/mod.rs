pub mod parser;

use std::fmt::Debug;

use crate::primitives::{TriangleMesh, TriangleIndex};

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