#![allow(non_snake_case)]
use itertools::Itertools;
use proconio::{fastout, input};

#[fastout]
fn main() {
  input! {n:usize,k:usize,r:[usize;n]}
  println!(
    "{}",
    (0..n)
      .map(|i| 1..=r[i])
      .multi_cartesian_product()
      .filter_map(|x| if x.iter().sum::<usize>() % k == 0 {
        Some(x.iter().join(" "))
      } else {
        None
      })
      .join("\n")
  )
}
