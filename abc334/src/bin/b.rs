#![allow(non_snake_case)]
use proconio::{fastout, input};

#[fastout]
fn main() {
  input! {a:i128,m:i128,mut l:i128,mut r:i128}
  l += m * (1 << 64) - a;
  r += m * (1 << 64) - a;
  println!("{}", r / m - l / m + (l % m == 0) as i128);
}
