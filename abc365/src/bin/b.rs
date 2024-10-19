#![allow(non_snake_case)]
use itertools::Itertools;
use proconio::{fastout, input};

#[fastout]
fn main() {
  input! {n:usize,a:[u32;n]}
  println!(
    "{}",
    (0..n).sorted_by_key(|&i| a[i]).rev().nth(1).unwrap() + 1
  );
}
