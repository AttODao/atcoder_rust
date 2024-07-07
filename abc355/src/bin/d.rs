#![allow(non_snake_case)]
use proconio::{fastout, input};
use superslice::*;

#[fastout]
fn main() {
  input! {n:usize,mut lr:[(u32,u32);n]}
  lr.sort_unstable_by_key(|(_, r)| *r);
  let mut ans = n * (n - 1) / 2;
  for &(l, _) in &lr {
    ans -= lr.lower_bound_by_key(&l, |(_, r)| *r);
  }
  println!("{ans}");
}
