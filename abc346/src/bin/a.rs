#![allow(non_snake_case)]
use itertools::Itertools;
use proconio::{fastout, input};

#[fastout]
fn main() {
  input! {a:[u32]}
  println!("{}", a.windows(2).map(|a| a[0] * a[1]).join(" "));
}
