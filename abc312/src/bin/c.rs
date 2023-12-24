#![allow(non_snake_case)]
use std::cmp::Reverse;

use proconio::{fastout, input};

#[fastout]
fn main() {
  input! {n:usize,m:usize,mut a:[u32;n],mut b:[u32;m]}

  a.push(0);
  b.push(1 << 30);
  a.sort();
  b.sort_by_key(|&b| Reverse(b));
  let (mut ng, mut ok) = (0, 1 << 30);
  while ok - ng > 1 {
    let mid = (ng + ok) / 2;
    if a.partition_point(|a| a <= &mid) >= b.partition_point(|b| b >= &mid) {
      ok = mid;
    } else {
      ng = mid;
    }
  }
  println!("{}", ok);
}
