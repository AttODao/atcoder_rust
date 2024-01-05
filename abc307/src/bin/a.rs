#![allow(non_snake_case)]
use itertools::Itertools;
use proconio::{fastout, input};

#[fastout]
fn main() {
  input! {n:usize,a:[[u64;7];n]}
  println!(
    "{}",
    a.into_iter().map(|a| a.into_iter().sum::<u64>()).join(" ")
  );
}
