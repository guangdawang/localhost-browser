//! 性能基准测试
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use port_browser::validator;

fn bench_port_validation(c: &mut Criterion) {
    let mut group = c.benchmark_group("validation");

    group.bench_function("valid_port", |b| {
        b.iter(|| validator::validate_port_str(black_box("8080")))
    });

    group.bench_function("invalid_port", |b| {
        b.iter(|| validator::validate_port_str(black_box("99999")))
    });

    group.bench_function("empty_port", |b| {
        b.iter(|| validator::validate_port_str(black_box("")))
    });

    group.finish();
}

criterion_group!(benches, bench_port_validation);
criterion_main!(benches);
