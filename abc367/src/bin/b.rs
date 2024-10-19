#![allow(non_snake_case)]
use proconio::{fastout, input};

#[fastout]
fn main() {
  input! {s:String}
  println!("{}", s.trim_end_matches("0").trim_end_matches("."));
}
