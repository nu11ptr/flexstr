use std::rc::Rc;
use std::sync::Arc;

use criterion::{criterion_group, criterion_main, Criterion};
use flexstr::{IntoFlexStr, ToAFlexStr, ToFlexStr};

const SMALL_STR: &str = "Inline";
const NORMAL_STR: &str = "This is a normal type string. It is a typical size for a basic message.";
// A little over 900 chars
const LARGE_STR: &str = "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod \
tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud \
exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in \
reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint \
occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum. \
Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore \
et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut \
aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse \
cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in \
culpa qui officia deserunt mollit anim id est laborum.";

pub fn create(c: &mut Criterion) {
    // Static and Inline
    c.bench_function("create_static_normal", |b| {
        b.iter(|| NORMAL_STR.into_flex_str())
    });
    c.bench_function("create_inline_small", |b| {
        b.iter(|| SMALL_STR.to_flex_str())
    });

    // Heap
    c.bench_function("create_heap_normal", |b| {
        b.iter(|| NORMAL_STR.to_flex_str())
    });
    c.bench_function("create_heap_large", |b| b.iter(|| LARGE_STR.to_flex_str()));

    // Heap (Arc)
    c.bench_function("create_heap_arc_normal", |b| {
        b.iter(|| NORMAL_STR.to_a_flex_str())
    });
    c.bench_function("create_heap_arc_large", |b| {
        b.iter(|| LARGE_STR.to_a_flex_str())
    });

    // String
    c.bench_function("create_string_small", |b| b.iter(|| SMALL_STR.to_string()));
    c.bench_function("create_string_normal", |b| {
        b.iter(|| NORMAL_STR.to_string())
    });
    c.bench_function("create_string_large", |b| b.iter(|| LARGE_STR.to_string()));

    // Rc
    c.bench_function("create_rc_small", |b| {
        b.iter(|| {
            let rc: Rc<str> = Rc::from(SMALL_STR);
            rc
        })
    });
    c.bench_function("create_rc_normal", |b| {
        b.iter(|| {
            let rc: Rc<str> = Rc::from(NORMAL_STR);
            rc
        })
    });
    c.bench_function("create_rc_large", |b| {
        b.iter(|| {
            let rc: Rc<str> = Rc::from(LARGE_STR);
            rc
        })
    });

    // Arc
    c.bench_function("create_arc_small", |b| {
        b.iter(|| {
            let arc: Arc<str> = Arc::from(SMALL_STR);
            arc
        })
    });
    c.bench_function("create_arc_normal", |b| {
        b.iter(|| {
            let arc: Arc<str> = Arc::from(NORMAL_STR);
            arc
        })
    });
    c.bench_function("create_arc_large", |b| {
        b.iter(|| {
            let arc: Arc<str> = Arc::from(LARGE_STR);
            arc
        })
    });
}

criterion_group!(benches, create);
criterion_main!(benches);
