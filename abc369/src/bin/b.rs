#![allow(non_snake_case)]
use itertools::Itertools;
use proconio::{fastout, input};

#[fastout]
fn main() {
  input! {a:[(u32,char)]}
  println!(
    "{}",
    a.iter()
      .filter(|&&(_, s)| s == 'L')
      .tuple_windows()
      .map(|(&(a, _), &(b, _))| u32::abs_diff(a, b))
      .sum::<u32>()
      + a
        .iter()
        .filter(|&&(_, s)| s == 'R')
        .tuple_windows()
        .map(|(&(a, _), &(b, _))| u32::abs_diff(a, b))
        .sum::<u32>()
  );
}
