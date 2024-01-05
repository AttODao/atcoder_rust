#![allow(non_snake_case)]
use itertools::Itertools;
use proconio::{fastout, input};

#[fastout]
fn main() {
  input! {_:usize,s:String}
  println!(
    "{}",
    s.chars().map(|c| c.to_string() + &c.to_string()).join("")
  );
}
