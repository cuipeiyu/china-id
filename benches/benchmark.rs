use criterion::{black_box, criterion_group, criterion_main, Criterion};

use china_id::ChinaId;

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("bench", |b| {
        b.iter(|| {
            let id = ChinaId::new(black_box("43102220200101133X"));

            let _ = id.valid();
            let _ = id.adcode();
            let _ = id.birthday();
            let _ = id.gender();
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
