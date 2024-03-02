#![allow(non_snake_case)]
use proconio::{fastout, input};

#[fastout]
fn main() {
  input! {s:String}
  println!("{}4", s[0..(s.len() - 1)].to_string());
}
