#![allow(non_snake_case)]
use itertools::Itertools;
use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
  input! {s:[Chars]}
  println!(
    "{}",
    (0..s.iter().map(Vec::len).max().unwrap())
      .map(|i| s
        .iter()
        .rev()
        .map(|s| s.get(i).unwrap_or(&'*'))
        .collect::<String>()
        .trim_end_matches('*')
        .to_string())
      .join("\n")
  );
}
