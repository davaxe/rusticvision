pub mod camera;
pub mod object;
pub mod parser;

use crate::primitives::TriangleMesh;

use camera::Camera;
use object::Object;

pub struct Scene<'this> {
    objects: Vec<Object<'this>>,
    camera: Camera,
    triangle_mesh: &'this TriangleMesh,
}

impl<'this> Scene<'this> {
    pub fn with_default_camera(
        triangle_mesh: &'this TriangleMesh,
        objects: Vec<Object<'this>>,
    ) -> Self {
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
