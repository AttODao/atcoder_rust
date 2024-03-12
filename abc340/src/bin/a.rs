#![allow(non_snake_case)]
use itertools::Itertools;
use proconio::{fastout, input};

#[fastout]
fn main() {
  input! {a:u32,b:u32,d:u32}
  println!("{}", (0..=(b - a) / d).map(|i| a + i * d).join(" "));
}
