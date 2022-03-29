use criterion::{black_box, criterion_group, criterion_main, Criterion};
use fastdiv::FastDiv;

pub fn criterion_benchmark(c: &mut Criterion) {
    {
        let d: u32 = black_box(3);
        let n: u32 = 4;
        let precomputed = d.precompute_div();

        c.bench_function("fast div u32", |b| {
            b.iter(|| black_box(black_box(n).fast_div(precomputed)))
        });
        c.bench_function("slow div u32", |b| b.iter(|| black_box(black_box(n) / d)));
        c.bench_function("const div u32", |b| {
            b.iter(|| black_box(black_box(n) / 3))
        });

        c.bench_function("fast mod u32", |b| {
            b.iter(|| black_box(black_box(n).fast_mod(precomputed, d)))
        });
        c.bench_function("slow mod u32", |b| b.iter(|| black_box(black_box(n) % d)));
        c.bench_function("const mod u32", |b| {
            b.iter(|| black_box(black_box(n) % 3))
        });
    }
    {
        let d: u64 = black_box(3);
        let n: u64 = 2_u64.pow(32);
        let precomputed = d.precompute_div();

        c.bench_function("fast div u64", |b| {
            b.iter(|| black_box(black_box(n).fast_div(precomputed)))
        });
        c.bench_function("slow div u64", |b| b.iter(|| black_box(black_box(n) / d)));
        c.bench_function("const div u64", |b| {
            b.iter(|| black_box(black_box(n) / 3))
        });

        c.bench_function("fast mod u64", |b| {
            b.iter(|| black_box(black_box(n).fast_mod(precomputed, d)))
        });
        c.bench_function("slow mod u64", |b| b.iter(|| black_box(black_box(n) % d)));
        c.bench_function("const mod u64", |b| {
            b.iter(|| black_box(black_box(n) % 3))
        });
    }
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
