#![allow(non_snake_case)]

use itertools::Itertools;
use proconio::{fastout, input};

#[fastout]
fn main() {
  input! {n:usize,q:usize,mut r:[u64;n],x:[u64;q]}

  r.push(0);
  let sum = r
    .iter()
    .sorted()
    .scan(0, |sum, r| {
      *sum += r;
      Some(*sum)
    })
    .collect_vec();
  for x in x {
    println!("{}", sum.partition_point(|&sum| sum <= x) - 1);
  }
}
