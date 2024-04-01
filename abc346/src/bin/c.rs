#![allow(non_snake_case)]
use std::collections::HashSet;

use proconio::{fastout, input};

#[fastout]
fn main() {
  input! {n:usize,k:usize,a:[usize;n]}
  let mut set = HashSet::new();
  let mut ans = k * (k + 1) / 2;
  for a in a {
    if a <= k && set.insert(a) {
      ans -= a;
    }
  }
  println!("{ans}");
}
