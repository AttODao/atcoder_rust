#![allow(non_snake_case)]
use proconio::{fastout, input};

#[fastout]
fn main() {
  input! {y:u32}
  println!(
    "{}",
    match (y % 4, y % 100, y % 400) {
      (_, _, 0) => 366,
      (_, 0, _) => 365,
      (0, _, _) => 366,
      _ => 365,
    }
  )
}
