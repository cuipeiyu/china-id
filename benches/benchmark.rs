use criterion::{black_box, criterion_group, criterion_main, Criterion};

use china_id::ChinaId;

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("parse", |b| b.iter(|| {
        ChinaId::new(black_box("43102220200101133X")).unwrap()
    }));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
