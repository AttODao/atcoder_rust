#![allow(non_snake_case)]
use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
  input! {s:Chars}
  println!(
    "{}",
    if s.windows(2).all(|c| c[0] <= c[1]) {
      "Yes"
    } else {
      "No"
    }
  );
}
