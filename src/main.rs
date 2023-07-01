use rusticvision::prelude::*;

fn main() {
    let tracer = RayTracer::new()
        .directory("test")
        .obj_file("test.obj")
        .camera_position(7.0, 1.9, 0.0)
        .camera_target(0.0, 1.9, 0.0)
        .resolution(512, 512)
        .sample_count(250)
        .recursion_depth(4);

    tracer.render_save("test.png");
}
