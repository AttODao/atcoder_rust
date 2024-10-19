#![allow(non_snake_case)]
use proconio::{fastout, input};

#[fastout]
fn main() {
  input! {n:u32,t:u32,a:u32}
  if 2 * t > n || 2 * a > n {
    println!("Yes");
  } else {
    println!("No");
  }
}
