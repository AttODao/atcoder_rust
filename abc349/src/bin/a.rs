#![allow(non_snake_case)]
use proconio::{fastout, input};

#[fastout]
fn main() {
  input! {n:usize,a:[i32;n-1]}
  println!("{}", -a.iter().sum::<i32>());
}
