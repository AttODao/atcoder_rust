#![allow(non_snake_case)]
use proconio::{fastout, input};

#[fastout]
fn main() {
  input! {n:usize,a:[i64;n]}
  let mut p = vec![0; n + 1];
  for i in 0..n {
    p[i + 1] = p[i] + a[i];
  }
  println!("{}", p[n] - p.into_iter().min().unwrap());
}
