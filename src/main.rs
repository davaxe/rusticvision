use rusticvision::prelude::*;

fn main() {
    {
        let tracer = RayTracer::new()
            .directory("test")
            .obj_file("test_scene.obj")
            .camera_position(7.0, 1.9, 0.0)
            .camera_target(0.0, 1.9, 0.0)
            .resolution(512, 512)
            .sample_count(64)
            .recursion_depth(6);

        tracer.render_save("test2.png");
    }
}
