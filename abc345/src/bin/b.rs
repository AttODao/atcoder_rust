#![allow(non_snake_case)]
use proconio::{fastout, input};

#[fastout]
fn main() {
  input! {x:i64}
  println!("{}", if x > 0 { (x + 9) / 10 } else { x / 10 });
}
