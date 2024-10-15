#![allow(non_snake_case)]

use proconio::{fastout, input};

#[fastout]
fn main() {
  input! {r:u32,g:u32,b:u32,c:String}
  println!(
    "{}",
    match c.as_str() {
      "Red" => g.min(b),
      "Green" => r.min(b),
      "Blue" => r.min(g),
      _ => unreachable!(),
    }
  )
}
