#![allow(non_snake_case)]
use proconio::{fastout, input};

#[fastout]
fn main() {
  input! {n:usize,m:usize,h:[usize;n]}
  let mut s = 0;
  for i in 0..n {
    s += h[i];
    if s > m {
      println!("{i}");
      return;
    }
  }
  println!("{n}");
}
