#![allow(non_snake_case)]
use proconio::{fastout, input};

#[fastout]
fn main() {
  input! {a:u64,b:u64}
  println!("{}", a.pow(b as u32) + b.pow(a as u32));
}
