#![allow(non_snake_case)]
use proconio::{fastout, input};

#[fastout]
fn main() {
  input! {n:String}
  let len = n.len();
  println!(
    "{}",
    if len < 3 {
      n
    } else {
      n[0..3].to_string() + &"0".repeat(n.len() - 3)
    }
  )
}
