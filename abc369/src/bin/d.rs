#![allow(non_snake_case)]
use std::iter::once;

use proconio::{fastout, input};

#[fastout]
fn main() {
  input! {a:[i64]}
  println!(
    "{}",
    a.into_iter()
      .chain(once(0))
      .fold((0, i64::MIN), |(dp0, dp1), a| (
        dp0.max(dp1 + a * 2),
        dp1.max(dp0 + a)
      ))
      .0
  )
}
