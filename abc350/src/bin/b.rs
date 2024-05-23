#![allow(non_snake_case)]
use itertools::Itertools;
use proconio::{fastout, input, marker::Usize1};

#[fastout]
fn main() {
  input! {n:usize,t:[Usize1]}
  println!(
    "{}",
    n - t.iter().counts().values().filter(|&i| i & 1 == 1).count()
  )
}
