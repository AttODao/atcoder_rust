#![allow(non_snake_case)]
use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
  input! {_:usize,s:Chars}
  let mut table = vec![false; 3];
  for (i, s) in s.into_iter().enumerate() {
    table[s as usize - 'A' as usize] = true;
    if table.iter().all(|&t| t) {
      println!("{}", i + 1);
      return;
    }
  }
}
