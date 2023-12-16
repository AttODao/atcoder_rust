#![allow(non_snake_case)]
use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
  input! {s:Chars,t:Chars}

  let ls = (5 + s[0] as u32 - s[1] as u32) % 5;
  let lt = (5 + t[0] as u32 - t[1] as u32) % 5;
  if ls == lt || (ls + lt) == 5 {
    println!("Yes");
  } else {
    println!("No");
  }
}
