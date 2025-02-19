#![allow(incomplete_features)]
#![allow(clippy::all)]
#![feature(test)]
#![feature(const_generics)]

extern crate test;

use test::{black_box, Bencher};

use staticvec::*;

use std::io::Write;

#[bench]
fn extend_with_constant(b: &mut Bencher) {
  let mut v = StaticVec::<u8, 512>::new();
  let cap = v.capacity();
  b.iter(|| {
    v.clear();
    let constant = black_box(1);
    v.extend((0..cap).map(move |_| constant));
    v[511]
  });
  b.bytes = v.capacity() as u64;
}

#[bench]
fn extend_with_range(b: &mut Bencher) {
  let mut v = StaticVec::<u8, 512>::new();
  let cap = v.capacity();
  b.iter(|| {
    v.clear();
    let range = 0..cap;
    v.extend(range.map(|x| black_box(x as u8)));
    v[511]
  });
  b.bytes = v.capacity() as u64;
}

#[bench]
fn extend_with_slice(b: &mut Bencher) {
  let mut v = StaticVec::<u8, 512>::new();
  let data = [1; 512];
  b.iter(|| {
    v.clear();
    let iter = data.iter().map(|&x| x);
    v.extend(iter);
    v[511]
  });
  b.bytes = v.capacity() as u64;
}

#[bench]
fn extend_with_write(b: &mut Bencher) {
  let mut v = StaticVec::<u8, 512>::new();
  let data = [1; 512];
  b.iter(|| {
    v.clear();
    v.write(&data[..]).ok();
    v[511]
  });
  b.bytes = v.capacity() as u64;
}

#[bench]
fn extend_from_slice(b: &mut Bencher) {
  let mut v = StaticVec::<u8, 512>::new();
  let data = [1; 512];
  b.iter(|| {
    v.clear();
    v.try_extend_from_slice(&data).ok();
    v[511]
  });
  b.bytes = v.capacity() as u64;
}
