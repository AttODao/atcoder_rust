#![allow(non_snake_case)]
use std::cmp::Ordering;

use proconio::{fastout, input};

#[fastout]
fn main() {
  input! {xy:[(u32,u32)]}
  let (x, y): (Vec<_>, Vec<_>) = xy.into_iter().unzip();
  println!(
    "{}",
    match x.iter().sum::<u32>().cmp(&y.iter().sum::<u32>()) {
      Ordering::Greater => "Takahashi",
      Ordering::Equal => "Draw",
      Ordering::Less => "Aoki",
    }
  )
}
