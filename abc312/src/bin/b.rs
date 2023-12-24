#![allow(non_snake_case)]
use itertools::iproduct;
use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
  input! {n:usize,m:usize,s:[Chars;n]}

  for (i, j) in iproduct!(0..n - 8, 0..m - 8) {
    if iproduct!(0..4, 0..4).all(|(x, y)| {
      let target = if x == 3 || y == 3 { '.' } else { '#' };
      s[i + x][j + y] == target && s[i + 8 - x][j + 8 - y] == target
    }) {
      println!("{} {}", i + 1, j + 1);
    }
  }
}
