use std::rc::Rc;
use std::sync::Arc;

use compact_str::CompactStr;
use criterion::{criterion_group, criterion_main, BatchSize, BenchmarkId, Criterion};
use flexstr::{AFlexStr, FlexStr, Repeat};
use kstring::KString;
use smartstring::{LazyCompact, SmartString};
use smol_str::SmolStr;

macro_rules! static_clone {
    ($($name:expr, $setup:expr),+) => {
        fn static_clone(c: &mut Criterion) {
            let mut group = c.benchmark_group("Clone - Literal");
            const STR: &'static str = "The length of this string is irrelevant!";

            $(let id = BenchmarkId::new($name, STR.len());
            group.bench_function(id, |b| {
                b.iter_batched(|| $setup, |s| s.clone(), BatchSize::SmallInput)
            });)+

            group.finish();
        }
    };
}

static_clone!(
    "String",
    STR.to_string(),
    "FlexStr",
    FlexStr::from_static(STR),
    "AFlexStr",
    AFlexStr::from_static(STR)
);

macro_rules! clone {
    ($($name:expr, $setup:expr),+) => {
        fn clone(c: &mut Criterion) {
            let mut group = c.benchmark_group("Clone - Computed");
            let lengths = vec![0usize, 10, 20, 100, 1000, 16384];

            for len in lengths {
                $(let id = BenchmarkId::new($name, len);
                group.bench_function(id, |b| {
                    b.iter_batched(|| $setup(len), |s| s.clone(), BatchSize::SmallInput)
                });)+
            }

            group.finish();
        }
    };
}

clone!(
    "String",
    |len| "x".repeat(len),
    "Rc<str>",
    |len| -> Rc<str> { "x".repeat(len).into() },
    "Arc<str>",
    |len| -> Arc<str> { "x".repeat(len).into() },
    "FlexStr",
    |len| -> FlexStr { "x".repeat_n(len) },
    "AFlexStr",
    |len| -> AFlexStr { "x".repeat_n(len) },
    "CompactStr",
    |len| -> CompactStr { "x".repeat(len).into() },
    "KString",
    |len| -> KString { "x".repeat(len).into() },
    "SmartString",
    |len| -> SmartString<LazyCompact> { "x".repeat(len).into() },
    "SmolStr",
    |len| -> SmolStr { "x".repeat(len).into() }
);

criterion_group!(benches, static_clone, clone);
criterion_main!(benches);
