#![allow(non_snake_case)]
use proconio::{fastout, input};

#[fastout]
fn main() {
  input! {a:u8,b:u8}
  println!(
    "{}",
    if [(1, 2), (2, 3), (4, 5), (5, 6), (7, 8), (8, 9)].contains(&(a, b)) {
      "Yes"
    } else {
      "No"
    }
  );
}
