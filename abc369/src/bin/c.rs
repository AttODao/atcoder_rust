#![allow(non_snake_case)]
use std::{i32, iter::once};

use itertools::Itertools;
use proconio::{fastout, input};

#[fastout]
fn main() {
  input! {n:usize,a:[i32;n]}
  println!(
    "{}",
    a.into_iter()
      .tuple_windows()
      .map(|(a1, a2)| a1 - a2)
      .tuple_windows()
      .map(|(a1, a2)| a1 == a2)
      .chain(once(false))
      .fold((2 * n - 1, 0), |(ans, l), d| if d {
        (ans, l + 1)
      } else {
        (ans + l * (l + 1) / 2, 0)
      })
      .0
  )
}
