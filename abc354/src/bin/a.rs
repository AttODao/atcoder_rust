#![allow(non_snake_case)]
use proconio::{fastout, input};

#[fastout]
fn main() {
  input! {h:u32}
  println!("{}", 32 - (h + 1).leading_zeros());
}
