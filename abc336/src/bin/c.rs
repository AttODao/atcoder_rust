#![allow(non_snake_case)]
use itertools::Itertools;
use proconio::{fastout, input};

#[fastout]
fn main() {
  input! {mut n:u64}
  n -= 1;
  if n == 0 {
    println!("0");
    return;
  }
  let mut ans_rev = "".to_string();
  while n > 0 {
    ans_rev.push(char::from_digit((n % 5) as u32 * 2, 10).unwrap());
    n /= 5;
  }
  println!("{}", ans_rev.chars().rev().join(""));
}
