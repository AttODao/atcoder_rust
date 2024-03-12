#![allow(non_snake_case)]
use itertools::Itertools;
use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
  input! {s:Chars}
  let mut cnt = [0; 26];
  for c in s {
    cnt[c as usize - 'a' as usize] += 1;
  }
  let max = cnt.iter().max().unwrap();
  println!(
    "{}",
    (cnt.iter().position(|c| c == max).unwrap() + 'a' as usize) as u8 as char
  );
}
