#![allow(non_snake_case)]
use proconio::{fastout, input};

#[fastout]
fn main() {
  input! {l:u8,r:u8}
  println!(
    "{}",
    match (l, r) {
      (1, 0) => "Yes",
      (0, 1) => "No",
      _ => "Invalid",
    }
  )
}
