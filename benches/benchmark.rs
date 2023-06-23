use criterion::{criterion_group, criterion_main, Criterion};

use rusticvision::prelude::*;

fn render_monkey() {
    let tracer = RayTracer::new()
        .directory("test")
        .obj_file("monkey.obj")
        .camera_position(0.0, 0.0, -5.0)
        .resolution(800, 600)
        .sample_count(1)
        .recursion_depth(1);

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
