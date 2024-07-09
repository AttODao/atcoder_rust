#![allow(non_snake_case)]
use itertools::Itertools;
use proconio::{fastout, input};

#[fastout]
fn main() {
  input! {n:usize,a:[usize;2*n]}
  let mut ans = 0;
  for (p, _, q) in a.iter().tuple_windows() {
    if p == q {
      ans += 1;
    }
  }
  println!("{ans}");
}
