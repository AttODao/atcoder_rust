#![allow(non_snake_case)]
use proconio::{fastout, input};

#[fastout]
fn main() {
  input! {s:String}
  let n = s[3..].parse::<u16>().unwrap();
  println!(
    "{}",
    if n > 0 && n < 350 && n != 316 {
      "Yes"
    } else {
      "No"
    }
  );
}
