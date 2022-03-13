use std::rc::Rc;
use std::sync::Arc;

use compact_str::CompactStr;
use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use flexstr::{AFlexStr, FlexStr, Repeat, ToAFlexStr, ToFlexStr};
use kstring::KString;
use smartstring::{LazyCompact, SmartString};
use smol_str::SmolStr;

fn static_create(c: &mut Criterion) {
    let mut group = c.benchmark_group("Create and Destroy - Literal");
    const STRING: &'static str = "The length of this string is irrelevant!";

    let id = BenchmarkId::new("String", STRING.len());
    group.bench_function(id, |b| b.iter(|| STRING.to_string()));

    let id = BenchmarkId::new("FlexStr", STRING.len());
    group.bench_function(id, |b| b.iter(|| FlexStr::from_static(STRING)));

    let id = BenchmarkId::new("AFlexStr", STRING.len());
    group.bench_function(id, |b| b.iter(|| AFlexStr::from_static(STRING)));

    group.finish();
}

fn create(c: &mut Criterion) {
    let mut group = c.benchmark_group("Create and Destroy - Computed");

    let strings: Vec<FlexStr> = vec![0usize, 10, 20, 100, 1000, 16384]
        .into_iter()
        .map(|n| "x".repeat_n(n))
        .collect();

    for string in strings {
        let id = BenchmarkId::new("String", string.len());
        group.bench_with_input(id, string.as_str(), |b, s| b.iter(|| String::from(s)));

        let id = BenchmarkId::new("Rc<str>", string.len());
        group.bench_with_input(id, string.as_str(), |b, s| b.iter(|| <Rc<str>>::from(s)));

        let id = BenchmarkId::new("Arc<str>", string.len());
        group.bench_with_input(id, string.as_str(), |b, s| b.iter(|| <Arc<str>>::from(s)));

        let id = BenchmarkId::new("FlexStr", string.len());
        group.bench_with_input(id, string.as_str(), |b, s| b.iter(|| s.to_flex_str()));

        let id = BenchmarkId::new("AFlexStr", string.len());
        group.bench_with_input(id, string.as_str(), |b, s| b.iter(|| s.to_a_flex_str()));

        let id = BenchmarkId::new("CompactStr", string.len());
        group.bench_with_input(id, string.as_str(), |b, s| b.iter(|| CompactStr::new(s)));

        let id = BenchmarkId::new("KString", string.len());
        group.bench_with_input(id, string.as_str(), |b, s| b.iter(|| KString::from_ref(s)));

        let id = BenchmarkId::new("SmartString", string.len());
        group.bench_with_input(id, string.as_str(), |b, s| {
            b.iter(|| SmartString::<LazyCompact>::from(s))
        });

        let id = BenchmarkId::new("SmolStr", string.len());
        group.bench_with_input(id, string.as_str(), |b, s| b.iter(|| SmolStr::new(s)));
    }

    group.finish();
}

criterion_group!(benches, static_create, create);
criterion_main!(benches);
