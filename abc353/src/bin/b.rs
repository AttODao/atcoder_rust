#![allow(non_snake_case)]
use proconio::{fastout, input};

#[fastout]
fn main() {
  input! {n:usize,k:usize,a:[usize;n]}
  println!(
    "{}",
    a.into_iter()
      .fold((0, k), |(ans, r), a| {
        if a > r {
          (ans + 1, k - a)
        } else {
          (ans, r - a)
        }
      })
      .0
      + 1
  );
}
