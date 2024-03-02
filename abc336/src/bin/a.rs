#![allow(non_snake_case)]
use proconio::{fastout, input};

#[fastout]
fn main() {
  input! {n:usize}
  println!("L{}ng", "o".repeat(n));
}
