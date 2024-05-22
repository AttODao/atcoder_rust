#![allow(non_snake_case)]
use std::cmp::Reverse;

use itertools::Itertools;
use proconio::{fastout, input};

#[fastout]
fn main() {
  input! {xy:[(i32,i32)]}
  for &(x1, y1) in &xy {
    println!(
      "{}",
      xy.iter()
        .enumerate()
        .map(|(i, &(x2, y2))| ((x1 - x2) * (x1 - x2) + (y1 - y2) * (y1 - y2), Reverse(i)))
        .position_max()
        .unwrap()
        + 1
    );
  }
}
