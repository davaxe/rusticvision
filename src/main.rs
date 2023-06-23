use rusticvision::prelude::*;

fn main() {
    let tracer = RayTracer::new()
        .directory("test")
        .obj_file("monkey.obj")
        .camera_position(0.0, 0.0, -5.0)
        .resolution(800, 600)
        .sample_count(1)
        .recursion_depth(1);

    tracer.render_save("test.png")
}
