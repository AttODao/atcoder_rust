#![allow(non_snake_case)]
use proconio::{fastout, input};

#[fastout]
fn main() {
  input! {n:usize}
  let mut ans = 0;
  for i in (1..1000000).map(|i| i * i * i).take_while(|&i| i <= n) {
    let s = i.to_string();
    if s == s.chars().rev().collect::<String>() {
      ans = i;
    }
  }
  println!("{}", ans);
}
