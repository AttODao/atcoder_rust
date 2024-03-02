#![allow(non_snake_case)]
use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
  input! {x:u64,y:u64,z:u64,s:Chars}
  let mut dp = (0, z);
  for c in s {
    dp = if c == 'a' {
      (dp.0 + x, dp.1 + y)
    } else {
      (dp.0 + y, dp.1 + x)
    };
    dp = (dp.0.min(dp.1 + z), dp.1.min(dp.0 + z));
  }
  println!("{}", dp.0.min(dp.1));
}
