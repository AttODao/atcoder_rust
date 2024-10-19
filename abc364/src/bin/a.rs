#![allow(non_snake_case)]
use itertools::Itertools;
use proconio::{fastout, input};

#[fastout]
fn main() {
  input! {s:[String]}
  if s
    .iter()
    .rev()
    .skip(1)
    .tuple_windows()
    .any(|(a, b)| a == "sweet" && b == "sweet")
  {
    println!("No");
  } else {
    println!("Yes");
  }
}
