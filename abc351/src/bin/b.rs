#![allow(non_snake_case)]
use itertools::iproduct;
use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
  input! {n:usize,a:[Chars;n],b:[Chars;n]}
  for (i, j) in iproduct!(0..n, 0..n) {
    if a[i][j] != b[i][j] {
      println!("{} {}", i + 1, j + 1);
      return;
    }
  }
}
