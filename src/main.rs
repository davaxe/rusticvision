use rusticvision::prelude::*;

fn main() {
    {
        let tracer = RayTracer::new()
            .directory("test")
            .obj_file("test.obj")
            .camera_position(7.0, 1.9, 0.0)
            .camera_target(0.0, 1.9, 0.0)
            .resolution(1000, 1000)
            .sample_count(1)
            .recursion_depth(1);

        tracer.render_gpu();
    }
}
