use std::borrow::Cow;
use std::hint::black_box;
use std::rc::Rc;
use std::sync::Arc;

use criterion::{BatchSize, BenchmarkId, Criterion, criterion_group, criterion_main};

macro_rules! import {
    ($func:ident, $group:expr,$($name:expr, $op:expr),+) => {
        fn $func(c: &mut Criterion) {
            let mut group = c.benchmark_group($group);

            let strings: Vec<String> = vec![0usize, 10, 20, 100, 500]
                .into_iter()
                .map(|n| String::from("x").repeat(n))
                .collect();

            for string in strings {
                let len = string.len();
            $(
                let id = BenchmarkId::new($name, len);

                group.bench_function(id, |b|
                    b.iter_batched(|| string.clone(), |s| {
                        let s = $op(s);
                        black_box(&s);
                    }, BatchSize::SmallInput)
                );
            )+
            }

            group.finish();
        }
    };
}

import!(
    import_owned_and_destroy,
    "Import as owned and destroy",
    "Rc<str>",
    |s: String| -> Rc<str> { s.into() },
    "Arc<str>",
    |s: String| -> Arc<str> { s.into() },
    "Cow<'_, str>",
    |s: String| -> Cow<'_, str> { s.into() },
    "FlexStr 0.9 (SharedStr)",
    |s: String| -> flexstr09::SharedStr { s.into() },
    "FlexStr 0.9 (LocalStr)",
    |s: String| -> flexstr09::LocalStr { s.into() },
    "InlineFlexStr 0.1 (InlineStr)",
    |s: String| -> Result<inline_flexstr::InlineFlexStr<str>, _> { s.as_str().try_into() },
    "FlexStr 0.10 (SharedStr - As is)",
    |s: String| -> flexstr::SharedStr { s.into() },
    "FlexStr 0.10 (LocalStr - As is)",
    |s: String| -> flexstr::LocalStr { s.into() },
    "FlexStr 0.10 (SharedStr - Optimized)",
    |s: String| {
        let s: flexstr::SharedStr = s.into();
        s.optimize()
    },
    "FlexStr 0.10 (LocalStr - Optimized)",
    |s: String| {
        let s: flexstr::LocalStr = s.into();
        s.optimize()
    }
);

import!(
    import_borrowed_own_and_destroy,
    "Import as borrowed, own and destroy",
    "Cow<'_, str>",
    |s: String| {
        let s: Cow<'_, str> = s.as_str().into();
        s.into_owned()
    },
    "FlexStr 0.10 (SharedStr)",
    |s: String| {
        let s: flexstr::SharedStr = s.as_str().into();
        s.into_owned()
    },
    "FlexStr 0.10 (LocalStr)",
    |s: String| {
        let s: flexstr::LocalStr = s.as_str().into();
        s.into_owned()
    }
);

criterion_group!(
    benches,
    import_owned_and_destroy,
    import_borrowed_own_and_destroy
);
criterion_main!(benches);
