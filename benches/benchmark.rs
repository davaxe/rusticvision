use criterion::{criterion_group, criterion_main, Criterion};

use rusticvision::prelude::*;

fn render_monkey_many_samples() {
    let tracer = RayTracer::new()
        .directory("test")
        .obj_file("test.obj")
        .camera_position(7.0, 1.9, 0.0)
        .camera_target(0.0, 1.9, 0.0)
        .resolution(250, 250)
        .sample_count(1000)
        .recursion_depth(4);

    tracer.render();
}

fn render_monkey_few_samples() {
    let tracer = RayTracer::new()
        .directory("test")
        .obj_file("test.obj")
        .camera_position(7.0, 1.9, 0.0)
        .camera_target(0.0, 1.9, 0.0)
        .resolution(1581, 1581 )
        .sample_count(60)
        .recursion_depth(4);

    tracer.render();
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("render", |b| b.iter(render_monkey_many_samples));
    c.bench_function("render few samples", |b| {
        b.iter(render_monkey_few_samples)
    });
}

criterion_group!(
    name = benches;
    config = Criterion::default().sample_size(10);
    targets = criterion_benchmark
);
criterion_main!(benches);
