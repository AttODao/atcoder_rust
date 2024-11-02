#![allow(non_snake_case)]
use proconio::{fastout, input};

#[fastout]
fn main() {
  input! {a:u32,b:u32}
  if a == b {
    println!("1");
  } else if (a + b) % 2 == 0 {
    println!("3");
  } else {
    println!("2");
  }
}
