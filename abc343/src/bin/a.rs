#![allow(non_snake_case)]
use proconio::{fastout, input};

#[fastout]
fn main() {
  input! {a:u8,b:u8}
  println!("{}", if a + b == 0 { 1 } else { 0 });
}
