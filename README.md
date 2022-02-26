# flexstr

A flexible, simple to use, immutable, clone-efficient `String` replacement for 
Rust

## Overview

Rust is great, but it's `String` type is not optimized for typical string 
use cases, but instead as a mutable string buffer. Most string use cases 
don't modify their string contents, often need to copy strings around as if 
they were cheap like integers, typically concatenate instead of modify, and 
often end up being cloned with identical contents. Additionally, `String` 
isn't able wrap a string literal without additional allocation and copying. 
Rust really needs a 3rd string type to unify usage of both literals and 
allocated strings in typical use cases. This crate creates a new string type 
that is optimized for those use cases, while retaining the usage simplicity of
`String`.

This type is not inherently "better" than `String`, but different. It is a 
higher level type, that can at times mean higher overhead. It really 
depends on the use case.

## Features

* Optimized for immutability and cheap cloning
* Allows for multiple ownership of the same string memory contents
* It is very simple to use
* The same size as a `String` (24 bytes on 64-bit, 12 bytes on 32-bit)
* Optional serde serialization support (feature = "serde")
* Allows for simple conditional ownership scenarios (borrows can turn into 
  ownership without allocation/copying)
* Serves as a universal string type (unifying literals and allocated strings)
* Doesn't allocate for literals and short strings (64-bit: up to 22 bytes)
* Provides easy access to `&str` via dereference
* Isn't much more expensive than `String` in non-optimal use cases

## Types

* `FlexStr`
    * Wrapper type for string literals (`&'static str`), inlined strings 
      (`InlineFlexStr`), or an `Rc` wrapped `str` 
    * NOT `Send` or `Sync` (due to usage of `Rc`)
* `AFlexStr`
    * Equivalent to `FlexStr` but uses `Arc` instead of `Rc` for the wrapped 
      `str`
    * Both `Send` and `Sync`
* `InlineFlexStr`
    * Custom inline string type holding up to 22 bytes (on 64-bit platforms)
    * Used automatically as needed by `FlexStr` and `AFlexStr` - not typically 
      used directly

## Usage

### Hello World

```rust
use flexstr::IntoFlexStr;

fn main() {
  // Literal - no copying or allocation
  let hello = "world!".into_flexstr();
  
  println!("Hello {world}");
}
```

### Conversions

```rust
use flexstr::{IntoAFlexStr, IntoFlexStr, ToFlexStr};

fn main() {
    // From literal - no copying or allocation
    // NOTE: `to_flexstr` will copy, so use `into_flexstr` for literals
    let literal = "literal".into_flexstr();
    
    // From borrowed string - Copied into inline string
    let owned = "inlined".to_string();
    let str_to_inlined = (&owned).to_flexstr();

    // From borrowed String - copied into `str` wrapped in `Rc`
    let owned = "A bit too long to be inlined!!!".to_string();
    let str_to_wrapped = (&owned).to_flexstr();
    
    // From String - copied into inline string (`String` storage released)
    let inlined = "inlined".to_string().into_flexstr();

    // From String - `str` wrapped in `Rc` (`String` storage released)
    let counted = "A bit too long to be inlined!!!".to_string().into_flexstr();
   
    // *** If you want a Send/Sync type you need `AFlexStr` instead ***

    // From FlexStr wrapped literal - no copying or allocation
    let literal = literal.into_a_flexstr();
    
    // From FlexStr inlined string - no allocation
    let inlined = inlined.into_a_flexstr();
    
    // From FlexStr `Rc` wrapped `str` - copies into `str` wrapped in `Arc`
    let counted = counted.into_a_flexstr();
}
```

### Borrowing

Works just like `String`

NOTE: The only benefit to passing as a `&str` is more compatibility with 
existing code. By passing as a `&FlexStr` instead, we retain the possibility 
of cheap multi ownership (see below).

```rust
use flexstr::FlexStr;

fn my_func(str: &FlexStr) {
    println!("Borrowed string: {str}");
}

fn main() {
    // Literal - no copy or allocation
    let str: FlexStr = "my string".into();
    my_func(&str);
}
```

### Passing FlexStr to Conditional Ownership Functions

This has always been a confusing situation in Rust, but it is easy with 
`FlexStr` since multi ownership is cheap.

```rust
use flexstr::{IntoFlexStr, FlexStr};

struct MyStruct {
    s: FlexStr
}

impl MyStruct {
    fn to_own_or_not_to_own(s: &FlexStr) -> Self {
        let s = if s == "own_me" {
            // Since a wrapped literal, no copy or allocation
            s.clone()
        } else {
            // Wrapped literal - no copy or allocation
            "own_me".into()
        };

        Self { s }
    }
}

fn main() {
    // Wrapped literals - no copy or allocation
    let s = "borrow me".into_flexstr();
    let s2 = "own me".into_flexstr();

    let struct1 = MyStruct::to_own_or_not_to_own(&s);
    let struct2 = MyStruct::to_own_or_not_to_own(&s2);

    assert_eq!(s2, struct1.s);
    assert_eq!(s2, struct2.s);
}
```

## Performance Characteristics

NOTE: No benchmarking has yet been done

* Clones are cheap and never allocate
    * At minimum, they are just a copy of the enum and at max an additional 
      reference count increment
* Literals are just wrapped when used with `into()` and never copied
* Calling `into()` on a `String` will result in an inline string (if 
  short) otherwise copied into a `str` wrapped in `Rc`/`Arc` 
  (which will allocate, copy, and then release original `String` storage)
* `into_string()` and `into_a_string()` are equivalent to calling `into()` 
  on both literals and `String` (they are present primarily for `let` 
  bindings without needing to declare type)
* `to_flexstr()` and `to_a_flexstr()` are meant for the on-boarding of borrowed 
  strings and always copy into either an inline string (for short strings) or 
  an `Rc`/`Arc` wrapped `str` (which will allocate)
* `to_string` always copies into a new `String`
* Conversions back and forth between `AFlexStr` and `FlexStr` using `into()` 
  are cheap when using wrapped literals or inlined strings
    * Inlined strings and wrapped literals just create a new enum wrapper
    * Reference counted wrapped strings will always require an allocation 
      and copy for the  new `Rc` or `Arc`

## Negatives

There is no free lunch:

* Due to usage of `Rc` (or `Arc`), when onboading `String` it will need to 
  reallocate and copy
* Due to the enum wrapper, every string operation has the overhead of an extra
  branching operation
* Since `FlexStr is not `Send` or `Sync`, thre is a need to consider 
  single-threaded   (`FlexStr`) and multi-threaded (`AFlexStr`) use cases and 
  convert accordingly

## Open Issues / TODO

* Reinvent common macros like `format!` (and `aformat!`) for creating
  strings to avoid need to go back and forth to `String`

## Status

This is currently Alpha quality and in heavy development. There is much testing 
and design work still needed. The API may break at any time.

## License

This project is licensed optionally under either:

* Apache License, Version 2.0, (LICENSE-APACHE
  or https://www.apache.org/licenses/LICENSE-2.0)
* MIT license (LICENSE-MIT or https://opensource.org/licenses/MIT)
