#![allow(non_snake_case)]
use proconio::{fastout, input};

#[fastout]
fn main() {
  input! {n:usize,a:usize,t:[usize;n]}
  let mut now = 0;
  for t in t {
    now = t.max(now) + a;
    println!("{now}");
  }
}
