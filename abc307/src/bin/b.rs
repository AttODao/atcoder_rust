#![allow(non_snake_case)]
use itertools::Itertools;
use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
  input! {s:[Chars]}
  for t in s.into_iter().permutations(2).map(|s| s.concat()) {
    if t.iter().collect_vec() == t.iter().rev().collect_vec() {
      println!("Yes");
      return;
    }
  }
  println!("No");
}
