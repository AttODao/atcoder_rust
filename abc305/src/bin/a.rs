#![allow(non_snake_case)]
use proconio::{fastout, input};

#[fastout]
fn main() {
  input! {n:u32}
  println!("{}", (n + 2) / 5 * 5);
}
