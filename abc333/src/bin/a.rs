#![allow(non_snake_case)]
use proconio::{fastout, input};

#[fastout]
fn main() {
  input! {n:usize}
  for _ in 0..n {
    print!("{}", n);
  }
}
