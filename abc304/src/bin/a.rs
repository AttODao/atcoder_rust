#![allow(non_snake_case)]
use itertools::Itertools;
use proconio::{fastout, input};

#[fastout]
fn main() {
  input! {n:usize,sa:[(String,u32);n]}
  let min_index = sa.iter().position_min_by_key(|&(_, a)| a).unwrap();
  println!("{}", (0..n).map(|i| &sa[(min_index + i) % n].0).join("\n"));
}
