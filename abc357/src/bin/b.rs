#![allow(non_snake_case)]
use proconio::{fastout, input};

#[fastout]
fn main() {
  input! {s:String}
  println!(
    "{}",
    if s.chars().filter(|c| c.is_lowercase()).count() * 2 > s.len() {
      s.to_lowercase()
    } else {
      s.to_uppercase()
    }
  );
}
