# stringy

A simple to use, immutable, clone-efficient `String` replacement for Rust

## Overview

Rust is awesome, but it's `String` type is not optimized for many typical use
cases, but instead is optimized as a  mutable string buffer. Most string use 
cases don't modify the string contents, often treat strings as if they were cheap 
like primitives, typically concatenate instead of modify, and often end up 
being cloned with identical contents. This crate attempts to create a new string 
type that is optimized for typical string use cases, while retaining the 
simplicity of `String`.

## Goals

Create a string type that:

* Is optimized for immutability and cheap cloning
* Allows for multiple ownership of the same string memory contents
* Is very simple to use
* For literals and short strings, allow for zero allocation
* Provide easy access to `&str` via dereference
* Allow for seamless coexistence with `String`
* Serve as a single string type (unifying literals and `String`)
* Isn't much more expensive than `String` in non-optimal use cases

## Negatives

There is no free lunch:

* Due to being an enum wrapper + padding/alignment it ends up being 8 bytes
  larger than `String` on 64-bit platforms (24 vs 32 bytes)
* Due to usage of `Rc` (or `Arc`) it requires two allocations instead of one
  when using the ref counted enum variant
* Due to the enum wrapper, every string operation has the overhead of a
  branching operation

I don't consider any of these terribly serious in most of my use cases, but
call them out in case these pose an issue to your workload.

## Open Issues / TODO

* Find/create a new ref count type that inlines string contents (to avoid
  double allocation)
* Reinvent common macros like `format!` for creating strings to avoid need to go 
  back and forth to `String`

## Types

* `Stringy`
    * Primary type - however, since it might use `Rc` it is not `Send`/`Sync`
* `AStringy`
    * Equivalent to `Stringy` but uses `Arc` instead of `Rc` (is therefore `Send`/`Sync`)

## How to Use

For the most part, you use it like any other string type. It works like you 
would expect. You can dereference into `&str`, concatenate strings with `+`, 
make an efficient copy with `.clone()`, etc.

* Creation:
    * From `String` or `&'static str` = `.into()`
        * String itself will not ever cause an allocation
            * Possible allocation from `Rc` or `Arc` for `String` (if not 
              inlined)
    * Wrap `String` = `.wrap_as_stringy()` or `wrap_as_astringy`
        * Doesn't consider inlining which is useful if you wish to efficiently 
          retrieve the original `String` later without allocation
            * Allocates since it will use `Rc` or `Arc`
    * From `&str` or `&String` = `.to_stringy()` or `.to_astringy()`
        * Will allocate and copy borrowed string


* Conversion back into a `String`:
    * `to_string` = Creates a brand new `String` and copies contents into it
        * Original `Stringy` preserved
    * `into_string` = Attempts to return the original `String` if possible 
    (`Rc`/`Arc` wrapped with single ownership) otherwise it works like 
      `to_string` 
        * Original `Stringy` is consumed 
    * `try_into_string` = Attempts to return the original `String` if possible 
      (`Rc`/`Arc` wrapped with single ownership) otherwise it returns a copy of 
      the original `Stringy` 
      `Stringy` as error
        * Original `Stringy` is consumed  

## Usage

### Basic Usage

```rust
fn main() {
    // Wrapped literal - no allocation
    let literal: Stringy = "literal".into();
    
    // Copied borrowed string - inlined
    let owned = "inlined".to_string();
    let str_to_inlined = (&*owned).to_stringy();

    // Copied borrowed String - wrapped in Rc
    let owned = "A bit too long to be inlined!!!".to_string();
    let str_to_wrapped = (&*owned).to_stringy();
    
    // Inlined string - `String` allocation released
    let inlined: Stringy = "inlined".to_string().into();
    
    // Wrapped string - `String` wrapped in `Rc`
    let wrapped: Stringy = "A bit too long to be inlined!!!".to_string().into();

    // *** If you want a Send/Sync type you need `AStringy` instead ***

    // Wrapped literal - no allocation
    let literal: AStringy = literal.into();
    
    // Inlined string - no allocation
    let inlined: Stringy = inlined.into();
    
    // Thread-safe wrapped string - `String` wrapped in `Arc`
    let wrapped: AStringy = wrapped.into();
}
```

### Borrowing

Works just like `String`. There is no real benefit to passing as `&str` as 
you can always deference inside the function. By passing in as `&Stringy` 
you retain the option for cheap conditional ownership via `clone()`.

```rust
fn my_func(str: &Stringy) {
    println!("Borrowed string: {}", str);
}

fn main() {
    let str: Stringy = "my string".into();
    my_func(&str);
}
```

### Conditional Ownership

This has always been a confusing situation in Rust, but is easy with `Stringy` 
since multi ownership is cheap.

```rust
struct MyStruct {
    s: Stringy
}

impl MyStruct {
    fn to_own_or_not_to_own(s: &Stringy) -> Self {
        let s = if &*s == "own_me" {
            s.clone()
        } else {
            // Wrapped literal - no allocation
            "own_me".into()
        };

        Self { s }
    }
}

fn main() {
    // Wrapped literal - no allocation
    let s = "borrow me".into();
    // Inlined string - `String` allocation released
    let s2 = "own me".to_string().into();

    let struct1 = MyStruct::to_own_or_not_to_own(s.clone());
    let struct2 = MyStruct::to_own_or_not_to_own(s2.clone());

    assert_eq!(s2, struct1.str);
    assert_eq!(s2, struct2.str);
}
```

## License

This project is licensed optionally under either:

* Apache License, Version 2.0, (LICENSE-APACHE
  or https://www.apache.org/licenses/LICENSE-2.0)
* MIT license (LICENSE-MIT or https://opensource.org/licenses/MIT)
