#![allow(non_snake_case)]
use proconio::{fastout, input};

#[fastout]
fn main() {
  input! {r:u32}
  println!("{}", (r / 100 + 1) * 100 - r);
}
