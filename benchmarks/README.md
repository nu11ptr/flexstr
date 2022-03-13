# Benchmarks

## Table of Contents

- [Environment](#environment)
- [Benchmark Pitfalls](#benchmark-pitfalls)
- [Benchmark Results](#benchmark-results)
    - [Create and Destroy - Literal](#create-and-destroy---literal)
    - [Create and Destroy - Computed](#create-and-destroy---computed)
    - [Clone - Literal](#clone---literal)
    - [Clone - Computed](#clone---computed)
    - [Convert](#convert)

## Environment

Benchmarks were run on:

```bash
$ cargo +nightly version -v
cargo 1.61.0-nightly (65c8266 2022-03-09)
release: 1.61.0-nightly
commit-hash: 65c82664263feddc5fe2d424be0993c28d46377a
commit-date: 2022-03-09
host: x86_64-unknown-linux-gnu
libgit2: 1.4.1 (sys:0.14.1 vendored)
libcurl: 7.80.0-DEV (sys:0.4.51+curl-7.80.0 vendored ssl:OpenSSL/1.1.1m)
os: Pop!_OS 20.04 (focal) [64-bit]
```

## Benchmark Pitfalls

The more testing I do, the more I'm convinced that microbenchmarks are nearly impossible to do accurately. I find
that simply moving the order of the benchmarks can make LARGE differences. I also see sudden large jumps and drop offs
even without doing anything at all. Using `black_box` seems just as sketchy, and without writing a book, find it creates
the same problems just skewed slightly differently. Nevermind `nightly` vs `stable`. Due to all this, please take these
with a grain of salt (maybe 5). Many of these are inaccurate or just plain wrong.

That said, they are not totally worthless. We can look for trends and patterns, and benchmarking has revealed some
performance surprises that resulted in beneficial code changes. The key things to watch for are (these will also give clues
when benchmarks are impossibly wrong):

* At size 0, we would expect "empty string" detection to kick in - should be as fast as a constant more or less
* For literal tests, we are just testing how fast we can return the item - all work done at compile time
* For sizes 22 and under (on 64-bit), these are all inlined - we would expect these to be faster than heap allocations
* For static and inlining, there are zero code differences between `FlexStr` and `AFlexStr` and we shouldn't expect any
performance difference at all (even though large differences are shown often!)

## Benchmark Results

### Create and Destroy - Literal

This just demonstrates the benefits of having a constant vs. heap allocating the constant as `String` is forced to do

|          | `String`                | `AFlexStr`                      | `FlexStr`                        |
|:---------|:------------------------|:--------------------------------|:-------------------------------- |
| **`40`** | `7.58 ns` (1.00x)       | `0.57 ns` (✅ **13.22x faster**) | `0.57 ns` (✅ **13.41x faster**)  |

### Create and Destroy - Computed

* String sizes of 10 and 20 are inlining, so gets a boost
* An ever so slight penalty for the wrapper on heap allocations
* String sizes of 0 are just empty string constants and any variation here is likely not meaningful

|             | `String`                  | `Rc<str>`                        | `Arc<str>`                       | `AFlexStr`                       | `FlexStr`                         |
|:------------|:--------------------------|:---------------------------------|:---------------------------------|:---------------------------------|:--------------------------------- |
| **`0`**     | `0.42 ns` (1.00x)         | `10.12 ns` (❌ *23.82x slower*)   | `10.82 ns` (❌ *25.46x slower*)   | `0.64 ns` (❌ *1.50x slower*)     | `1.06 ns` (❌ *2.50x slower*)      |
| **`10`**    | `9.91 ns` (1.00x)         | `10.11 ns` (❌ *1.02x slower*)    | `10.54 ns` (❌ *1.06x slower*)    | `6.32 ns` (✅ **1.57x faster**)   | `4.87 ns` (✅ **2.04x faster**)    |
| **`20`**    | `9.67 ns` (1.00x)         | `10.13 ns` (❌ *1.05x slower*)    | `10.39 ns` (❌ *1.07x slower*)    | `6.46 ns` (✅ **1.50x faster**)   | `6.35 ns` (✅ **1.52x faster**)    |
| **`100`**   | `10.32 ns` (1.00x)        | `10.45 ns` (❌ *1.01x slower*)    | `10.84 ns` (❌ *1.05x slower*)    | `11.08 ns` (❌ *1.07x slower*)    | `10.32 ns` (✅ **1.00x faster**)   |
| **`1000`**  | `13.57 ns` (1.00x)        | `14.31 ns` (❌ *1.05x slower*)    | `15.21 ns` (❌ *1.12x slower*)    | `14.74 ns` (❌ *1.09x slower*)    | `14.37 ns` (❌ *1.06x slower*)     |
| **`16384`** | `138.21 ns` (1.00x)       | `139.61 ns` (❌ *1.01x slower*)   | `189.43 ns` (❌ *1.37x slower*)   | `140.66 ns` (❌ *1.02x slower*)   | `182.79 ns` (❌ *1.32x slower*)    |

### Clone - Literal

This again just demonstrates the benefits of having a constant vs. heap allocating the constant as `String` is forced to do.
`AFlexStr` being much slower here is likely not correct in the real world as the code is identical to `FlexStr`

|          | `String`                 | `AFlexStr`                      | `FlexStr`                       |
|:---------|:-------------------------|:--------------------------------|:------------------------------- |
| **`40`** | `11.52 ns` (1.00x)       | `11.98 ns` (❌ *1.04x slower*)   | `4.26 ns` (✅ **2.70x faster**)  |

### Clone - Computed

* The benefits of simply copying a wrapper and possibly a ref count increment are apparent in `FlexStr`
* The 10 and 20 sizes being 4x slower makes zero sense - this is compiler derived `Clone` code that literally does one
less step than `Rc` derived `Clone` code, so we would expect it to be the same or faster. We also don't see this deviation
 in `AFlexStr`
* `AFlexStr` is nothing more than an enum wrapper over `Arc<str>` for sizes 100 and above, so it being ~5x slower
than a plain `Arc<str>` is very odd to say the least

|             | `String`                  | `Rc<str>`                       | `Arc<str>`                      | `AFlexStr`                       | `FlexStr`                         |
|:------------|:--------------------------|:--------------------------------|:--------------------------------|:---------------------------------|:--------------------------------- |
| **`0`**     | `6.80 ns` (1.00x)         | `0.70 ns` (✅ **9.66x faster**)  | `4.41 ns` (✅ **1.54x faster**)  | `12.32 ns` (❌ *1.81x slower*)    | `8.09 ns` (❌ *1.19x slower*)      |
| **`10`**    | `11.87 ns` (1.00x)        | `0.72 ns` (✅ **16.54x faster**) | `4.74 ns` (✅ **2.50x faster**)  | `12.38 ns` (❌ *1.04x slower*)    | `8.11 ns` (✅ **1.46x faster**)    |
| **`20`**    | `11.88 ns` (1.00x)        | `0.72 ns` (✅ **16.55x faster**) | `4.71 ns` (✅ **2.52x faster**)  | `12.38 ns` (❌ *1.04x slower*)    | `8.18 ns` (✅ **1.45x faster**)    |
| **`100`**   | `12.75 ns` (1.00x)        | `1.29 ns` (✅ **9.91x faster**)  | `5.07 ns` (✅ **2.51x faster**)  | `12.94 ns` (❌ *1.01x slower*)    | `2.02 ns` (✅ **6.31x faster**)    |
| **`1000`**  | `55.61 ns` (1.00x)        | `2.75 ns` (✅ **20.24x faster**) | `7.34 ns` (✅ **7.57x faster**)  | `13.10 ns` (✅ **4.24x faster**)  | `2.85 ns` (✅ **19.55x faster**)   |
| **`16384`** | `458.43 ns` (1.00x)       | `4.91 ns` (✅ **93.30x faster**) | `6.99 ns` (✅ **65.60x faster**) | `13.27 ns` (✅ **34.55x faster**) | `2.55 ns` (✅ **179.86x faster**)  |

### Convert

Thanks mostly to `ryu` and `itoa`, our primitive conversions handily outperforms `String`.

|            | `String`                  | `AFlexStr`                      | `FlexStr`                        |
|:-----------|:--------------------------|:--------------------------------|:-------------------------------- |
| **`bool`** | `16.58 ns` (1.00x)        | `1.06 ns` (✅ **15.63x faster**) | `0.67 ns` (✅ **24.83x faster**)  |
| **`char`** | `10.67 ns` (1.00x)        | `11.46 ns` (❌ *1.07x slower*)   | `13.45 ns` (❌ *1.26x slower*)    |
| **`i8`**   | `13.32 ns` (1.00x)        | `8.95 ns` (✅ **1.49x faster**)  | `10.05 ns` (✅ **1.32x faster**)  |
| **`i16`**  | `20.76 ns` (1.00x)        | `18.05 ns` (✅ **1.15x faster**) | `18.10 ns` (✅ **1.15x faster**)  |
| **`i32`**  | `31.73 ns` (1.00x)        | `14.64 ns` (✅ **2.17x faster**) | `14.55 ns` (✅ **2.18x faster**)  |
| **`i64`**  | `38.11 ns` (1.00x)        | `19.27 ns` (✅ **1.98x faster**) | `19.30 ns` (✅ **1.97x faster**)  |
| **`i128`** | `65.98 ns` (1.00x)        | `37.99 ns` (✅ **1.74x faster**) | `37.86 ns` (✅ **1.74x faster**)  |
| **`f32`**  | `112.65 ns` (1.00x)       | `24.85 ns` (✅ **4.53x faster**) | `25.05 ns` (✅ **4.50x faster**)  |
| **`f64`**  | `191.50 ns` (1.00x)       | `30.81 ns` (✅ **6.22x faster**) | `30.01 ns` (✅ **6.38x faster**)  |

---
Made with [criterion-table](https://github.com/nu11ptr/criterion-table)

