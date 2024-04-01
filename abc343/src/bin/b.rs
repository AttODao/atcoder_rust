#![allow(non_snake_case)]
use itertools::Itertools;
use proconio::{fastout, input};

#[fastout]
fn main() {
  input! {n:usize,a:[[u8;n];n]}
  println!(
    "{}",
    a.into_iter()
      .map(|a| (1..=n).filter(|&i| a[i - 1] == 1).join(" "))
      .join("\n")
  )
}
