#![allow(non_snake_case)]
use proconio::{fastout, input};

#[fastout]
fn main() {
  input! {n:i64,m:i64,p:i64}
  println!("{}", 0.max((n - m + p) / p))
}
