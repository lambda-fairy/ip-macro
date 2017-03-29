# ip-macro

Macros for writing IP addresses.

See <https://github.com/rust-lang/rfcs/issues/1926> for the background behind this crate.

## Rust nightly

This crate uses the recently implemented [procedural macros] feature, and so requires a nightly version of the compiler.

If you use [rustup][] (recommended), then you can install Rust Nightly using [these instructions][nightly howto].

[procedural macros]: https://github.com/rust-lang/rust/issues/38356
[rustup]: https://rustup.rs/
[nightly howto]: https://github.com/rust-lang-nursery/rustup.rs#working-with-nightly-rust

## Example

```rust
#![feature(proc_macro)]  // <- Don't forget this!!!

extern crate ip_macro;
use ip_macro::ip;

fn main() {
    println!("There's no place like {}", ip!("127.0.0.1"));
}
```
