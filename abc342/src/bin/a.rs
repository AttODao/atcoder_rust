#![allow(non_snake_case)]
use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
  input! {s:Chars}
  let c = if s[0] == s[1] || s[0] == s[2] {
    s[0]
  } else {
    s[1]
  };
  println!("{}", s.iter().position(|&d| d != c).unwrap() + 1);
}
