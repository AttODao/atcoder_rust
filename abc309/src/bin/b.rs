#![allow(non_snake_case)]
use itertools::Itertools;
use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
  input! {n:usize,a:[Chars;n]}
  let mut b = a.clone();
  for i in 0..n - 1 {
    b[0][i + 1] = a[0][i];
    b[i + 1][n - 1] = a[i][n - 1];
    b[n - 1][i] = a[n - 1][i + 1];
    b[i][0] = a[i + 1][0];
  }
  println!(
    "{}",
    b.into_iter()
      .map(|b| b.into_iter().collect::<String>())
      .join("\n")
  );
}
