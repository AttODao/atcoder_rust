#![allow(non_snake_case)]
use proconio::{fastout, input};

#[fastout]
fn main() {
  input! {n:usize,mut a:[u64;n],st:[(u64,u64);n-1]}
  for i in 0..n - 1 {
    a[i + 1] += a[i] / st[i].0 * st[i].1;
  }
  println!("{}", a[n - 1]);
}
