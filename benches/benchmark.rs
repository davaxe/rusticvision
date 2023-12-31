use criterion::{criterion_group, criterion_main, Criterion};

use rusticvision::prelude::*;

fn render_monkey() {
    let tracer = RayTracer::new()
        .directory("test")
        .obj_file("test_scene.obj")
        .camera_position(7.0, 1.9, 0.0)
        .camera_target(0.0, 1.9, 0.0)
        .resolution(50, 50)
        .sample_count(1000)
        .recursion_depth(4);

    tracer.render();
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("monkey render", |b| b.iter(render_monkey));
}

criterion_group!(
    name = benches;
    config = Criterion::default().sample_size(10);
    targets = criterion_benchmark
);
criterion_main!(benches);
