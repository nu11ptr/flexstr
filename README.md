# flexstr

[![Crate](https://img.shields.io/crates/v/flexstr)](https://crates.io/crates/flexstr)
[![Docs](https://docs.rs/flexstr/badge.svg)](https://docs.rs/flexstr)
[![Build](https://github.com/nu11ptr/flexstr/workflows/CI/badge.svg)](https://github.com/nu11ptr/flexstr/actions)
[![codecov](https://codecov.io/gh/nu11ptr/flexstr/branch/master/graph/badge.svg?token=yUZ8v2tKPd)](https://codecov.io/gh/nu11ptr/flexstr)

A flexible, simple to use, clone-efficient `String` replacement for Rust. It unifies borrowed, inlined, referenced counted and boxed strings into a single type.

## Overview

TL;DR - If you've used `Cow`, but you wish cloning owned strings was more performant and that being owned didn't always imply heap allocation, this crate might be what you are looking for. The operations are "lazy" (like `Cow`), and it tries not to do work the user is not expecting.

Longer:

There are now many clone efficient, inlined string crates available at this point, but this crate is a bit different. First, it is simple: it is just an enum, so you are always in control of what type of string it contains. Its basic semantics are modeled after the basic `Cow` in the stdlib. `Cow` is pretty handy, but borrowed/owned alone isn't always sufficient (this crate adds ref counted and inlined strings). Clones should ideally not allocate new space (until mutation is required). Also, it would be nice if short strings didn't allocate at all, since short strings are often prevalent. The goal was a unified string type that can handle just about any situation, and bring all those use cases together in a single type.

Each one of the enum variants excels at different use cases, but is brought together in a single type for maximum flexibility:

  - **Borrowed** - Clone/copy performance, memory efficiency and optimal `&str` interop
  - **Inlined** - Clone/copy performance for short strings, memory efficiency and mutability
  - **RefCounted** - Clone performance for long strings, memory efficiency and optimal `Arc<str>`/`Rc<str>` interop
  - **Boxed** - Mutability and optimal `String`/`Box<str>` interop

If you have used previous versions of this crate, you should be aware this new version is a ground up rewrite with a solidly different thought process, API and design. Even if the previous versions didn't match your needs, this one might. *Users should be aware that nearly all the string construction code is not yet present in this version.* The new way to do this (workaround?) is to do the work as a `String` and then import it into a `LocalStr` or `SharedStr`. Moving into and out of the boxed variant (`from_owned`) should be near zero cost.

Lastly, this might be the only inline/clone efficient string crate that is generic over all the Rust string types (`str`, `CStr`, `OsStr`, `Path`, `[u8]`).

## Features

* Simple: just an enum. You mostly already know how to use it.
* Borrowed, inlined, reference counted, and boxed strings in a single type
* O(1) clone
    * NOTE: first `clone` when variant is `Boxed` is O(n)
* Mutable (Copy-on-write under the hood, if necessary)
* Inlined string type can be used on its own
* Same size a a `String` (3 words wide, even inside an `Option`)
* Lazy instantiation (no unexpected allocations)
* No dependencies
    * NOTE: `serde` optional for serialization/deserialization
* Optional `no_std`
* Optional `safe` feature that forbids any `unsafe` usage
    * NOTE: This does induce a performance penalty, as would be expected
    * NOTE 2: `OsStr`/`Path` support on Windows requires at least one unsafe call (`win_min_unsafe` feature).
* Handles all Rust string types (`str`, `CStr`, `OsStr`, `Path`, `[u8]`)

## Cargo Features

* **safe** = Use all safe functions and add `forbid(unsafe_code)` (performance penalty)
* **std** = Use `std` (default)
* **serde** = add `serde` dependency and adds serialization/deserialization
* **win_min_unsafe** = enables the minimum necessary unsafe code on Windows to support `OsStr`/`Path`. No other othe string types or operating systems are impacted (implies `safe` feature).
    * NOTE: The code will refuse to compile if this is not specified when ALL the following conditions are true:
        * The `safe` feature is enabled
        * The `osstr` and/or `path` feature(s) are enabled
        * Compiling for Windows

### String Type Features:
* **str** = Enable `str`-based strings (default)
* **bytes** = Enable byte-based strings (`[u8]`)
* **cstr** = Enable `CStr`-based strings
* **osstr** = Enable `OsStr`-based strings
* **path** = Enable `Path`-based strings (implies `osstr` feature)

## Example

It is just an enum that looks like this - you can probably guess much of how it works just by looking at it:

```rust,ignore

// `S` is just the raw string type (typically `str`)
// `R` is just an `Arc` or a `Rc`.
pub enum FlexStr<'s, S, R> {
    Borrowed(&'s S),
    Inlined(InlineFlexStr<S>),
    RefCounted(R),
    Boxed(Box<S>),
}

// You would typically use it via one of the type aliases, for example:
pub type LocalStr<'s> = FlexStr<'s, str, Rc<str>>;
pub type SharedStr<'s> = FlexStr<'s, str, Arc<str>>;
```

Even that you don't really need to concern yourself with. You can just use it how you would expect a simple wrapper to behave.

```rust
use flexstr::*;

// This will be a "Borrowed" variant
let hello: SharedStr = "hello".into();
assert!(hello.is_borrowed());

// This will be a "Boxed" variant
let world: SharedStr = "world".to_string().into();
assert!(world.is_boxed());

// This is now "Inlined" (since it is short)
let hello = hello.into_owned();
assert!(hello.is_inlined());

// This is now "Inlined" as well (since it is short)
let world = world.optimize();
assert!(world.is_inlined());

println!("{hello} {world}");
```

## AI Usage

The code was written by hand with care (although AI tab completion was used). Any contributions should be completely understood by the contributor, whether AI assisted or not.

The tests on the otherhand were 90%+ generated by AI under my instruction. I've done a cursory review for sanity, but they need more work. Volunteers welcome.

## Status

This is currently experimental, however, I will be using this at a startup in production code, so it will become production ready at some point.

## Contributions

Contributions are welcome so long as they align to my vision for this crate. Currently, it does most of what I want it to do (outside of string construction and mutation, but I'm not ready to start on that yet).

## License

This project is licensed optionally under either:

* Apache License, Version 2.0, (LICENSE-APACHE
  or <https://www.apache.org/licenses/LICENSE-2.0>)
* MIT license (LICENSE-MIT or <https://opensource.org/licenses/MIT>)
