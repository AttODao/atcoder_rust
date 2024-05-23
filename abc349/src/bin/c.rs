#![allow(non_snake_case)]
use proconio::{fastout, input};

fn is_substring(s: String, t: String) -> bool {
  let mut si = s.chars();
  t.chars().all(|t| si.find(|&s| s == t).is_some())
}

#[fastout]
fn main() {
  input! {mut s:String,t:String}
  s.push('x');
  if is_substring(s, t.to_lowercase()) {
    println!("Yes");
  } else {
    println!("No");
  }
}
