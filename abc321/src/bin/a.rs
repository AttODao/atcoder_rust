#![allow(non_snake_case)]
use itertools::Itertools;
use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
  input! {n:Chars}
  println!(
    "{}",
    if n.into_iter().tuple_windows().all(|(a, b)| a > b) {
      "Yes"
    } else {
      "No"
    }
  );
}
