#![allow(non_snake_case)]
use proconio::{fastout, input};

#[fastout]
fn main() {
  input! {a:[u32;9],b:[u32;8]}
  println!("{}", a.iter().sum::<u32>() - b.iter().sum::<u32>() + 1);
}
