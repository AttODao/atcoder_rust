#![allow(non_snake_case)]
use proconio::{fastout, input};

#[fastout]
fn main() {
  input! {s:String}
  let t = s[..1].to_uppercase() + &s[1..].to_lowercase();
  println!("{}", if s == t { "Yes" } else { "No" });
}
