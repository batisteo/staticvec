[![Latest Version]][crates.io] ![Rustc Version nightly]

[Latest Version]: https://img.shields.io/crates/v/staticvec.svg
[crates.io]: https://crates.io/crates/staticvec
[Rustc Version nightly]: https://img.shields.io/badge/rustc-nightly-lightgray.svg
[![Build Status](https://travis-ci.com/slightlyoutofphase/staticvec.svg?branch=master)](https://travis-ci.com/slightlyoutofphase/staticvec)

Implements a fixed-capacity stack-allocated Vec alternative backed by an array, using const generics.

Note: the word "static" here is meant by the traditional definition of "unchanging" / "not dynamic" etc.

This crate does **not** use literal `static` variables for anything.

Fully `#![no_std]` compatible (with almost no loss of functionality) by setting
`default-features = false` for the `staticvec` dependency in your `Cargo.toml`.

Optional support for serialization and deserialization of the `StaticVec` struct
via `serde` is available by activating the `serde_support` feature.

Contributions/suggestions/etc. very welcome!

**Minimum supported Rust version:** due to the use of const generics, this is a nightly-only crate at the moment.

A basic usage example:

```rust
use staticvec::*;

fn main() {
  let mut v = StaticVec::<usize, 64>::new();
  for i in 0..v.capacity() {
    v.push(i);
  }
  for i in &v {
    println!("{}", i);
  }
  v.clear();
  v.insert(0, 47);
  v.insert(1, 48);
  v.insert(2, 49);
  v.insert(v.len() - 1, 50);
  v.insert(v.len() - 2, 51);
  v.insert(v.len() - 3, 52);
  for i in &v {
    println!("{}", i);
  }
  for i in &v.reversed().drain(2..4) {
    println!("{}", i);
  }
  while v.is_not_empty() {
    println!("{}", v.remove(0));
  }
  for f in staticvec![12.0, 14.0, 15.0, 16.0].iter().skip(2) {
    println!("{}", f);
  }
  for v in staticvec![
    staticvec![14, 12].sorted(),
    staticvec![18, 16].sorted(),
    staticvec![22, 20].sorted(),
    staticvec![26, 24].sorted(),
  ]
  .iter()
  .cloned::<StaticVec<usize, 2>>()
  {
    for i in &v {
      println!("{}", i);
    }
  }
}
```

**License:**

Licensed under either the <a href="LICENSE-MIT">MIT license</a> or version 2.0 of the <a href="LICENSE-APACHE">Apache License</a>. Your choice as to which!
Any source code contributions will be dual-licensed in the same fashion.
