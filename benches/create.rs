use std::rc::Rc;
use std::sync::Arc;

use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use flexstr::{AFlexStr, FlexStr, Repeat, ToAFlexStr, ToFlexStr};

fn static_create(c: &mut Criterion) {
    let mut group = c.benchmark_group("Create and Destroy (Literal)");

    const STRING: &'static str = "The length of this string is irrelevant!";
    group.bench_function("String/40", |b| b.iter(|| STRING.to_string()));
    group.bench_function("FlexStr/40", |b| b.iter(|| <FlexStr>::from_static(STRING)));
    group.bench_function("AFlexStr/40", |b| b.iter(|| AFlexStr::from_static(STRING)));

    group.finish();
}

fn create(c: &mut Criterion) {
    let mut group = c.benchmark_group("Create and Destroy");

    let strings: Vec<FlexStr> = vec![0usize, 10, 20, 100, 1000, 16384]
        .into_iter()
        .map(|n| "x".repeat_n(n))
        .collect();

    for string in strings {
        let id = BenchmarkId::new("String", string.len());
        group.bench_with_input(id, string.as_str(), |b, s| {
            b.iter(|| String::from(black_box(s)))
        });

        let id = BenchmarkId::new("Rc<str>", string.len());
        group.bench_with_input(id, string.as_str(), |b, s| {
            b.iter(|| <Rc<str>>::from(black_box(s)))
        });

        let id = BenchmarkId::new("Arc<str>", string.len());
        group.bench_with_input(id, string.as_str(), |b, s| {
            b.iter(|| <Arc<str>>::from(black_box(s)))
        });

        let id = BenchmarkId::new("FlexStr", string.len());
        group.bench_with_input(id, string.as_str(), |b, s| {
            b.iter(|| black_box(s).to_flex_str())
        });

        let id = BenchmarkId::new("AFlexStr", string.len());
        group.bench_with_input(id, string.as_str(), |b, s| {
            b.iter(|| black_box(s).to_a_flex_str())
        });
    }

    group.finish();
}

criterion_group!(benches, static_create, create);
criterion_main!(benches);
