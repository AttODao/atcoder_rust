#![allow(non_snake_case)]
use itertools::Itertools;
use proconio::{fastout, input, marker::Usize1};

#[fastout]
fn main() {
  input! {n:usize, t:u32, p:Usize1, l:[u32;n]}
  println!(
    "{}",
    t.saturating_sub(l.into_iter().sorted().rev().nth(p).unwrap())
  );
}
