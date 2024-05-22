#![allow(non_snake_case)]
use proconio::{fastout, input};

use itertools::Itertools;

#[fastout]
fn main() {
  input! {s:String}
  if s
    .chars()
    .counts()
    .values()
    .counts()
    .values()
    .all(|&v| v == 2)
  {
    println!("Yes");
  } else {
    println!("No");
  }
}
