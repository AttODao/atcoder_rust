#![allow(non_snake_case)]
use std::cmp::Ordering;

use itertools::Itertools;
use proconio::{fastout, input};

#[fastout]
fn main() {
  input! {ab:[(u64,u64)]}
  println!(
    "{}",
    ab.into_iter()
      .enumerate()
      .sorted_by(|(i, (a1, b1)), (j, (a2, b2))| (b1 * (a2 + b2), i).cmp(&(b2 * (a1 + b1), j)))
      .map(|(i, _)| i + 1)
      .join(" ")
  )
}
