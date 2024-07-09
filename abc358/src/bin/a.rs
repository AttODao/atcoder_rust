#![allow(non_snake_case)]
use proconio::{fastout, input};

#[fastout]
fn main() {
  input! {s:String,t:String}
  println!(
    "{}",
    if s == "AtCoder" && t == "Land" {
      "Yes"
    } else {
      "No"
    }
  )
}
