pub mod camera;
pub mod obj_parser;
pub mod object;

use std::collections::HashMap;

use crate::primitives::TriangleMesh;

use camera::Camera;
use object::Object;

pub struct Scene<'this> {
    objects: Vec<Object<'this>>,
    camera: Camera,
    triangle_mesh: &'this TriangleMesh,
}

impl<'this> Scene<'this> {
    pub fn new(triangle_mesh: &'this TriangleMesh, obj_map: &HashMap<String, String>, mat_map: &HashMap<String, usize>) -> Self {
        let objects = obj_parser::get_objects(triangle_mesh, obj_map, mat_map);
        let camera = Camera::default();
        Self {
            objects,
            camera,
            triangle_mesh,
        }

    }

    pub fn debug_print_objects(&self) {
        for object in &self.objects {
            println!("Object: {}", object.identifier);
            for triangle in &object.triangles {
                println!("\tTriangle: {:?}", triangle);
            }
            println!();
        }
    }
}
