#![allow(non_snake_case)]
use std::collections::BinaryHeap;

use proconio::{fastout, input};

#[fastout]
fn main() {
  input! {a:[usize]}
  let mut a = a.into_iter().collect::<BinaryHeap<_>>();
  let mut ans = 0;
  while let (Some(mut a1), Some(mut a2)) = (a.pop(), a.pop()) {
    a1 -= 1;
    a2 -= 1;
    if a1 > 0 {
      a.push(a1);
    }
    if a2 > 0 {
      a.push(a2);
    }
    ans += 1;
  }
  println!("{ans}")
}
