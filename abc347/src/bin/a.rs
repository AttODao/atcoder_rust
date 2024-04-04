#![allow(non_snake_case)]
use itertools::Itertools;
use proconio::{fastout, input};

#[fastout]
fn main() {
  input! {n:usize,k:u32,a:[u32;n]}
  println!(
    "{}",
    a.into_iter()
      .filter(|a| a % k == 0)
      .map(|a| a / k)
      .join(" ")
  );
}
