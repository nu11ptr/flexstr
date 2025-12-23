# Benchmarks

## Environment

The benchmarks were run on a Macbook Pro M1 Max with 32GB of RAM:

```bash
% cargo version -v
cargo 1.92.0 (344c4567c 2025-10-21)
release: 1.92.0
commit-hash: 344c4567c634a25837e3c3476aac08af84cf9203
commit-date: 2025-10-21
host: aarch64-apple-darwin
libgit2: 1.9.1 (sys:0.20.2 vendored)
libcurl: 8.7.1 (sys:0.4.83+curl-8.15.0 system ssl:(SecureTransport) LibreSSL/3.3.6)
ssl: OpenSSL 3.5.4 30 Sep 2025
os: Mac OS 15.7.1 [64-bit]
```

## Overview

At present, FlexStr is mostly just an enum wrapper (the exception being the inline string) that forwards its real world mostly to the stdlib. As such, the primary objective we looking for is:

1. That it does not add too much overhead to importing/instantiation over raw stdlib
1. That clone performance is fast enough to justify that extra overhead

The benchmark results meet these goals for me currently. The results are published at the link below.

## Results

The Criterion benchmark report can be found [here](https://nu11ptr.github.io/flexstr/criterion/report/)
