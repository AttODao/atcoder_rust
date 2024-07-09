#![allow(non_snake_case)]
use proconio::{fastout, input};

#[fastout]
fn main() {
  input! {s:String}
  println!(
    "{}",
    if s == "RMS" || s == "RSM" || s == "SRM" {
      "Yes"
    } else {
      "No"
    }
  );
}
