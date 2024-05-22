#![allow(non_snake_case)]
use proconio::{fastout, input};

#[fastout]
fn main() {
  input! {n:usize}
  println!("{}{}", "oox".repeat(n / 3), "o".repeat(n % 3));
}
