use criterion::{black_box, criterion_group, criterion_main, Criterion};

use fibonnaci_kreyren::fibonacci;

/*
  This is base benchmark made for consistency

  FIXME-BENCH: Make cargo to recognize `benches/rustlang` path instead of `benches/`

  FIXME-BENCH: Loosing around 70% of efficiency while sourcing fibonacci from lib.rs
*/

fn criterion_benchmark(c: &mut Criterion) -> () {
	c.bench_function("fibonacci 5", |b| b.iter(|| fibonacci().take(black_box(5))));
	c.bench_function("fibonacci 20", |b| b.iter(|| fibonacci().take(black_box(20))));
	c.bench_function("fibonacci 100", |b| b.iter(|| fibonacci().take(black_box(100))));
	c.bench_function("fibonacci 1000", |b| b.iter(|| fibonacci().take(black_box(1000))));
}

criterion_group! {
	name = benches;
	// NOTICE: Gitpod is noisy so we have to use more samples (https://github.com/gitpod-io/gitpod/issues/1217)
	// FIXME: Use more samples only if gitpod is used
	config = Criterion::default().sample_size(1000);
	targets = criterion_benchmark
}

criterion_main!(benches);