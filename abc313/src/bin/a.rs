#![allow(non_snake_case)]
use proconio::{fastout, input};

#[fastout]
fn main() {
  input! {p:[u32]}
  println!(
    "{}",
    (p.iter().skip(1).max().unwrap_or(&0) + 1).saturating_sub(p[0])
  );
}
