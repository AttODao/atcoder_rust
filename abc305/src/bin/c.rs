#![allow(non_snake_case)]
use itertools::iproduct;
use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
  input! {h:usize,w:usize,s:[Chars;h]}
  let a = (0..h).find(|&i| s[i].contains(&'#')).unwrap();
  let b = (0..h).rev().find(|&i| s[i].contains(&'#')).unwrap();
  let c = (0..w).find(|&j| (0..h).any(|i| s[i][j] == '#')).unwrap();
  let d = (0..w)
    .rev()
    .find(|&j| (0..h).any(|i| s[i][j] == '#'))
    .unwrap();
  for (i, j) in iproduct!(a..=b, c..=d) {
    if s[i][j] == '.' {
      println!("{} {}", i + 1, j + 1);
      return;
    }
  }
}
