use std::rc::Rc;
use std::sync::Arc;

use compact_str::CompactString;
use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use flexstr::{AFlexStr, AFlexStr_, FlexStr, FlexStr_, Repeat, ToAFlexStr, ToFlexStr};
use kstring::KString;
use smartstring::{LazyCompact, SmartString};
use smol_str::SmolStr;

const ITERATIONS: usize = 10_000;

// TODO: Add iterations and add back in
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

macro_rules! create {
    ($($name:expr, $op:expr),+) => {
        fn create(c: &mut Criterion) {
            let mut group = c.benchmark_group("Create and Destroy - Computed");

            let strings: Vec<FlexStr> = vec![0usize, 10, 20, 100, 1000, 16384]
                .into_iter()
                .map(|n| "x".repeat_n(n))
                .collect();

            for string in strings {
                $(let id = BenchmarkId::new($name, string.len());
                group.bench_with_input(id, string.as_str(), |b, s| b.iter(|| {
                    for _ in 0..ITERATIONS {
                        let s = $op(s);
                        black_box(&s);
                    }
                } ));)+
            }

            group.finish();
        }
    };
}

create!(
    "String",
    |s: &str| String::from(s),
    "Rc<str>",
    |s: &str| <Rc<str>>::from(s),
    "Arc<str>",
    |s: &str| <Arc<str>>::from(s),
    "FlexStr",
    |s: &str| s.to_flex_str(),
    "AFlexStr",
    |s: &str| s.to_a_flex_str(),
    "FlexStr_",
    |s: &str| FlexStr_::from(s),
    "AFlexStr_",
    |s: &str| AFlexStr_::from(s),
    "CompactString",
    |s: &str| CompactString::new(s),
    "KString",
    |s: &str| KString::from_ref(s),
    "SmartString",
    |s: &str| SmartString::<LazyCompact>::from(s),
    "SmolStr",
    |s: &str| SmolStr::new(s)
);

criterion_group!(benches, create);
criterion_main!(benches);
