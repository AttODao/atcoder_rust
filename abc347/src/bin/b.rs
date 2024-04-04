#![allow(non_snake_case)]
use std::collections::HashSet;

use itertools::Itertools;
use proconio::{fastout, input};

#[fastout]
fn main() {
  input! {s:String}
  println!(
    "{}",
    (0..=s.len())
      .tuple_combinations()
      .map(|(l, r)| &s[l..r])
      .collect::<HashSet<_>>()
      .len()
  );
}
