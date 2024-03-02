#![allow(non_snake_case)]
use proconio::{fastout, input};

#[fastout]
fn main() {
  input! {n:u32}
  for x in 0..=n {
    for y in 0..=(n - x) {
      for z in 0..=(n - x - y) {
        println!("{} {} {}", x, y, z);
      }
    }
  }
}
