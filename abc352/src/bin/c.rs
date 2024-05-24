#![allow(non_snake_case)]
use std::cmp::Reverse;

use proconio::{fastout, input};

#[fastout]
fn main() {
  input! {mut ab:[(usize,usize)]}
  ab.sort_by_key(|&(a, b)| Reverse(b - a));
  println!(
    "{}",
    ab.iter().map(|(a, _)| *a).sum::<usize>() - ab[0].0 + ab[0].1
  );
}
