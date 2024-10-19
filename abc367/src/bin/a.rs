#![allow(non_snake_case)]
use proconio::{fastout, input};

#[fastout]
fn main() {
  input! {a:u32,b:u32,c:u32}
  if (b < a && a < c) || (a < c && c < b) || (c < b && b < a) {
    println!("No");
  } else {
    println!("Yes");
  }
}
