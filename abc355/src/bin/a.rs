#![allow(non_snake_case)]
use proconio::{fastout, input};

#[fastout]
fn main() {
  input! {a:i8,b:i8}
  println!("{}", if a == b { -1 } else { 6 - a - b });
}
