# Benchmarks

## Table of Contents

- [Environment](#environment)
- [Third Party Crates](#third-party-crates)
- [Benchmark Pitfalls](#benchmark-pitfalls)
- [Benchmark Results](#benchmark-results)
    - [Create and Destroy - Computed](#create-and-destroy---computed)
    - [Clone - Computed](#clone---computed)

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

## Third Party Crates

I decided to include some popular 3rd party crates to include in the create/clone benchmarks. I try to be as fair as
possible, but I'm obviously inherently biased and even the choice of tests is biased. I'm primarily doing this as yet
another measuring stick on whether `FlexStr` is performing adequately or not (and now I see some places it is not).

Here are the 3rd party crates and their versions under test:

```
compact_str v0.3.1
flexstr v0.8.0
kstring v1.0.6
smartstring v1.0.0
smol_str v0.1.21
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

### Create and Destroy - Computed

* String sizes of 10 and 20 are inlining, so gets a boost
* An ever so slight penalty for the wrapper on heap allocations
* String sizes of 0 are just empty string constants and any variation here is likely not meaningful
* Overall I'm happy with how `FlexStr` performs here on inline string creation. I suspect the difference, however, to
 `CompactStr` and probably `SmartString` is just noise

|             | `String`                  | `FlexStr 0.8.0`                  | `AFlexStr 0.8.0`                 | `FlexStr 0.8.1`                  | `AFlexStr 0.8.1`                 | `FlexStr 0.9.0`                  | `AFlexStr 0.9.0`                  |
|:------------|:--------------------------|:---------------------------------|:---------------------------------|:---------------------------------|:---------------------------------|:---------------------------------|:--------------------------------- |
| **`0`**     | `6.40 us` (✅ **1.00x**)   | `10.68 us` (❌ *1.67x slower*)    | `10.67 us` (❌ *1.67x slower*)    | `6.44 us` (✅ **1.01x slower**)   | `6.39 us` (✅ **1.00x faster**)   | `6.41 us` (✅ **1.00x slower**)   | `6.37 us` (✅ **1.00x faster**)    |
| **`10`**    | `97.94 us` (✅ **1.00x**)  | `66.79 us` (✅ **1.47x faster**)  | `64.98 us` (✅ **1.51x faster**)  | `62.65 us` (✅ **1.56x faster**)  | `61.26 us` (✅ **1.60x faster**)  | `62.47 us` (✅ **1.57x faster**)  | `61.21 us` (✅ **1.60x faster**)   |
| **`20`**    | `95.23 us` (✅ **1.00x**)  | `101.27 us` (✅ **1.06x slower**) | `82.77 us` (✅ **1.15x faster**)  | `49.29 us` (🚀 **1.93x faster**)  | `46.59 us` (🚀 **2.04x faster**)  | `49.84 us` (🚀 **1.91x faster**)  | `47.01 us` (🚀 **2.03x faster**)   |
| **`100`**   | `101.38 us` (✅ **1.00x**) | `110.16 us` (✅ **1.09x slower**) | `121.07 us` (❌ *1.19x slower*)   | `110.21 us` (✅ **1.09x slower**) | `116.63 us` (❌ *1.15x slower*)   | `110.07 us` (✅ **1.09x slower**) | `116.61 us` (❌ *1.15x slower*)    |
| **`1000`**  | `136.76 us` (✅ **1.00x**) | `152.01 us` (❌ *1.11x slower*)   | `158.50 us` (❌ *1.16x slower*)   | `146.89 us` (✅ **1.07x slower**) | `154.38 us` (❌ *1.13x slower*)   | `145.83 us` (✅ **1.07x slower**) | `155.54 us` (❌ *1.14x slower*)    |
| **`16384`** | `1.85 ms` (✅ **1.00x**)   | `1.41 ms` (✅ **1.32x faster**)   | `1.46 ms` (✅ **1.27x faster**)   | `1.41 ms` (✅ **1.31x faster**)   | `1.46 ms` (✅ **1.27x faster**)   | `1.41 ms` (✅ **1.31x faster**)   | `1.45 ms` (✅ **1.28x faster**)    |

### Clone - Computed

* The benefits of simply copying a wrapper and possibly a ref count increment are apparent in `FlexStr`
* The 10 and 20 sizes being 4x slower makes zero sense - this is compiler derived `Clone` code that literally does one
less step than `Rc` derived `Clone` code, so we would expect it to be the same or faster. We also don't see this deviation
 in `AFlexStr`
* `AFlexStr` is nothing more than an enum wrapper over `Arc<str>` for sizes 100 and above, so it being ~5x slower
than a plain `Arc<str>` is very odd to say the least
* `FlexStr` falls pretty hard to `CompactStr` and `SmartString` on inline cloning. I'm not sure why, but I will be looking
into this
* At higher string sizes, the benefits of `Rc` and `Arc` used in `FlexStr` is very benefitical

|             | `String`                  | `FlexStr 0.8.0`                   | `AFlexStr 0.8.0`                  | `FlexStr 0.8.1`                   | `AFlexStr 0.8.1`                  | `FlexStr 0.9.0`                   | `AFlexStr 0.9.0`                   |
|:------------|:--------------------------|:----------------------------------|:----------------------------------|:----------------------------------|:----------------------------------|:----------------------------------|:---------------------------------- |
| **`0`**     | `40.59 us` (✅ **1.00x**)  | `15.01 us` (🚀 **2.70x faster**)   | `15.03 us` (🚀 **2.70x faster**)   | `10.76 us` (🚀 **3.77x faster**)   | `8.57 us` (🚀 **4.74x faster**)    | `10.65 us` (🚀 **3.81x faster**)   | `8.59 us` (🚀 **4.72x faster**)     |
| **`10`**    | `121.98 us` (✅ **1.00x**) | `51.83 us` (🚀 **2.35x faster**)   | `52.54 us` (🚀 **2.32x faster**)   | `8.55 us` (🚀 **14.26x faster**)   | `7.42 us` (🚀 **16.44x faster**)   | `8.61 us` (🚀 **14.17x faster**)   | `7.50 us` (🚀 **16.26x faster**)    |
| **`20`**    | `116.69 us` (✅ **1.00x**) | `53.67 us` (🚀 **2.17x faster**)   | `52.78 us` (🚀 **2.21x faster**)   | `8.54 us` (🚀 **13.67x faster**)   | `7.52 us` (🚀 **15.52x faster**)   | `8.70 us` (🚀 **13.42x faster**)   | `7.49 us` (🚀 **15.57x faster**)    |
| **`100`**   | `123.02 us` (✅ **1.00x**) | `17.24 us` (🚀 **7.13x faster**)   | `27.34 us` (🚀 **4.50x faster**)   | `12.92 us` (🚀 **9.52x faster**)   | `31.83 us` (🚀 **3.87x faster**)   | `12.91 us` (🚀 **9.53x faster**)   | `31.66 us` (🚀 **3.89x faster**)    |
| **`1000`**  | `151.23 us` (✅ **1.00x**) | `17.16 us` (🚀 **8.81x faster**)   | `27.36 us` (🚀 **5.53x faster**)   | `13.17 us` (🚀 **11.48x faster**)  | `27.75 us` (🚀 **5.45x faster**)   | `13.13 us` (🚀 **11.52x faster**)  | `27.76 us` (🚀 **5.45x faster**)    |
| **`16384`** | `3.21 ms` (✅ **1.00x**)   | `17.45 us` (🚀 **184.15x faster**) | `27.45 us` (🚀 **117.08x faster**) | `13.48 us` (🚀 **238.45x faster**) | `29.13 us` (🚀 **110.32x faster**) | `13.41 us` (🚀 **239.64x faster**) | `28.15 us` (🚀 **114.16x faster**)  |

---
Made with [criterion-table](https://github.com/nu11ptr/criterion-table)

