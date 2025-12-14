# flexstr

[![Crate](https://img.shields.io/crates/v/flexstr)](https://crates.io/crates/flexstr)
[![Docs](https://docs.rs/flexstr/badge.svg)](https://docs.rs/flexstr)
[![Build](https://github.com/nu11ptr/flexstr/workflows/CI/badge.svg)](https://github.com/nu11ptr/flexstr/actions)
[![codecov](https://codecov.io/gh/nu11ptr/flexstr/branch/master/graph/badge.svg?token=yUZ8v2tKPd)](https://codecov.io/gh/nu11ptr/flexstr)
[![MSRV](https://img.shields.io/badge/msrv-1.59-blue.svg)](https://crates.io/crates/flexstr)


A flexible, simple to use, immutable, clone-efficient `String` replacement for 
Rust. It unifies borrowed, inlined, and heap allocated strings into a single 
type.

## Overview

TL;DR - If you've used `Cow`, but you wish cloning owned strings was more performant and memory efficient, this crate might be for you. Our operations are "lazy" (just like `Cow`). We try not to do work the user is not expecting.

There are now many clone efficient, inlined string crates available, but this crate is a bit different. First, it is very simple: it is just an enum. I modeled its semantics after the basic `Cow` in the stdlib. `Cow` is pretty handy, but borrowed/owned isn't enough for what I needed. I wanted clones to not allocate new space, ideally ever. Also, I thought it would be nice if short strings didn't allocate at all, since I find short strings very prevalent. The goal was really a unified string type that can handle just about any situation (other than a mutable string buffer).

My previous attempts at writing this crate succombed a bit to much to "wouldn't it be cool", which is why this crate is a ground up rewrite, much simpler, and very pragmatic to typical string use cases.

Lastly, I think this might be the only inline/clone efficient string crate I'm aware of that is generic over all the Rust string types (`str`, `CStr`, `OsStr`, `Path`, `[u8]`).

## Features

* Simple: just an enum
* Borrowed, inlined, reference counted, and boxed strings in a singe type
* O(1) clone
    * NOTE: first `clone` when variant is `Boxed` is O(n)
* Same size a a `String` (3 words wide, even inside an `Option`)
* Lazy on import (no unexpected allocations)
* No dependencies
* Optional `no_std`
* Optional `safe` feature that forbids any `unsafe` usage
    * NOTE: This does induce a performance penalty
* Handles all Rust string types (`str`, `CStr`, `OsStr`, `Path`, `[u8]`)

## Cargo Features

* **safe** = Use all safe functions and add `forbid(unsafe_code)` (performance penalty)
* **std** = Use `std` (default)
* **str** = Enable `str`-based strings (default)
* **bytes** = Enable byte-based strings (`[u8]`)
* **cstr** = Enable `CStr`-based strings
* **osstr** = Enable `OsStr`-based strings
* **path** = Enable `Path`-based strings

## Example

It is just an enum that looks like this - you can probably guess much of how it works just by looking at it:

```rust,ignore

// `S` is just the string type (typically `str`)
// `R` is just an `Arc<str>` or `Rc<str>`.
pub enum Flex<'s, S, R> {
    Borrowed(&'s S),
    Inlined(InlineStr<S>),
    RefCounted(R),
    Boxed(Box<S>),
}

// Now we can declare some friendly types we can actually use
pub type LocalStr<'s> = Flex<'s, str, Rc<str>>;
pub type SharedStr<'s> = Flex<'s, str, Arc<str>>;
```

Even that you don't really need to concern yourself with. You can just use it how you would expect a simple wrapper to behave.

```rust
use flexstry::*;

// This will be a "Borrowed" variant
let hello: SharedStr<'_> = "hello".into();
assert!(hello.is_borrowed());

// This will be a "Boxed" variant
let world: SharedStr<'_> = "world".to_string().into();
assert!(world.is_boxed());

// This is now "Inlined" (since it is short)
let hello = hello.into_owned();
assert!(hello.is_inlined());

// This is now "Inlined" as well (since it is short)
let world = world.clone();
assert!(world.is_inlined());

println!("{hello} {world}");
```


## Status

This is currently experimental, however, I will be using this at a startup in production code, so it will become production ready.

## License

This project is licensed optionally under either:

* Apache License, Version 2.0, (LICENSE-APACHE
  or <https://www.apache.org/licenses/LICENSE-2.0>)
* MIT license (LICENSE-MIT or <https://opensource.org/licenses/MIT>)
