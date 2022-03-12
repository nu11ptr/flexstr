use criterion::{black_box, criterion_group, criterion_main, Criterion};
use flexstr::{ToAFlexStr, ToFlexStr};

macro_rules! min_nums {
    ($($name:ident, $type:ty),+) => {
        $(const $name: $type = <$type>::MIN;)+
    }
}

min_nums!(I8, i8, I16, i16, I32, i32, I64, i64, I128, i128, F32, f32, F64, f64);

const BOOL: bool = false;
const CHAR: char = '☺';

macro_rules! string_convert {
    ($($name:literal, $var:ident),+) => {
        pub fn string_convert(c: &mut Criterion) {
            $(c.bench_function($name, |b| {
                b.iter(|| black_box($var).to_string())
            });)+
        }
    };
}

string_convert!(
    "Convert/String/bool",
    BOOL,
    "Convert/String/char",
    CHAR,
    "Convert/String/i8",
    I8,
    "Convert/String/i16",
    I16,
    "Convert/String/i32",
    I32,
    "Convert/String/i64",
    I64,
    "Convert/String/i128",
    I128,
    "Convert/String/f32",
    F32,
    "Convert/String/f64",
    F64
);

macro_rules! flex_convert {
    ($($name:literal, $var:ident),+) => {
        pub fn flex_convert(c: &mut Criterion) {
            $(c.bench_function($name, |b| {
                b.iter(|| black_box($var).to_flex_str())
            });)+
        }
    };
}

flex_convert!(
    "Convert/FlexStr/bool",
    BOOL,
    "Convert/FlexStr/char",
    CHAR,
    "Convert/FlexStr/i8",
    I8,
    "Convert/FlexStr/i16",
    I16,
    "Convert/FlexStr/i32",
    I32,
    "Convert/FlexStr/i64",
    I64,
    "Convert/FlexStr/i128",
    I128,
    "Convert/FlexStr/f32",
    F32,
    "Convert/FlexStr/f64",
    F64
);

macro_rules! a_flex_convert {
    ($($name:literal, $var:ident),+) => {
        pub fn a_flex_convert(c: &mut Criterion) {
            $(c.bench_function($name, |b| {
                b.iter(|| black_box($var).to_a_flex_str())
            });)+
        }
    };
}

a_flex_convert!(
    "Convert/AFlexStr/bool",
    BOOL,
    "Convert/AFlexStr/char",
    CHAR,
    "Convert/AFlexStr/i8",
    I8,
    "Convert/AFlexStr/i16",
    I16,
    "Convert/AFlexStr/i32",
    I32,
    "Convert/AFlexStr/i64",
    I64,
    "Convert/AFlexStr/i128",
    I128,
    "Convert/AFlexStr/f32",
    F32,
    "Convert/AFlexStr/f64",
    F64
);

criterion_group!(benches, string_convert, flex_convert, a_flex_convert);
criterion_main!(benches);
