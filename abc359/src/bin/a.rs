#![allow(non_snake_case)]
use proconio::{fastout, input};

#[fastout]
fn main() {
  input! {s:[String]}
  println!("{}", s.iter().filter(|&s| s == "Takahashi").count())
}
