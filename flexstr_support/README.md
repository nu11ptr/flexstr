# flexstr_support

[![Crate](https://img.shields.io/crates/v/flexstr_support)](https://crates.io/crates/flexstr_support)
[![Docs](https://docs.rs/flexstr_support/badge.svg)](https://docs.rs/flexstr_support)
[![Build](https://github.com/nu11ptr/flexstr/workflows/CI/badge.svg)](https://github.com/nu11ptr/flexstr/actions)

Support crate for `flexstr` and `flexstr_support`. This probably isn't what you want unless you are trying to add your own string type to these crates.

## Cargo Features

* **safe** = Use all safe functions and add `forbid(unsafe_code)` (performance penalty)
* **std** = Use `std` (default)
* **serde** = add `serde` dependency and adds serialization/deserialization
* **win_min_unsafe** = enables the minimum necessary unsafe code on Windows to support `OsStr`/`Path`. No other string types or operating systems are impacted (implies `safe` feature).
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

## AI Usage

The code was written by hand with care (although AI tab completion was used). Any contributions should be completely understood by the contributor, whether AI assisted or not.

## Status

This is currently experimental, however, I will be using this at a startup in production code, so it will become production ready at some point.

## Contributions

Contributions are welcome so long as they align to my vision for this crate. Currently, it does most of what I want it to do (outside of string construction and mutation, but I'm not ready to start on that yet).

## License

This project is licensed optionally under either:

* Apache License, Version 2.0, (LICENSE-APACHE
  or <https://www.apache.org/licenses/LICENSE-2.0>)
* MIT license (LICENSE-MIT or <https://opensource.org/licenses/MIT>)
