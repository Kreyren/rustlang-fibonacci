use criterion::{black_box, criterion_group, criterion_main, Criterion};

use fibonnaci_kreyren::fibonacci;

/*
  This is base benchmark made for consistency

  FIXME-BENCH: Make cargo to recognize `benches/rustlang` path instead of `benches/`
*/

fn criterion_benchmark(c: &mut Criterion) -> () {
	c.bench_function("fib 5", |b| b.iter(|| fibonacci().take(black_box(5))));
	c.bench_function("fib 20", |b| b.iter(|| fibonacci().take(black_box(20))));
	c.bench_function("fib 100", |b| b.iter(|| fibonacci().take(black_box(100))));
	c.bench_function("fib 1000", |b| b.iter(|| fibonacci().take(black_box(1000))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);