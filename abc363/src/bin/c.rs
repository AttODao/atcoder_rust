#![allow(non_snake_case)]
use itertools::Itertools;
use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
  input! {n:usize,k:usize,s:Chars}
  let mut ans = 0;
  for t in s
    .into_iter()
    .permutations(n)
    .unique()
  {
    let mut flag = true;
    for t in t.windows(k) {
      if (0..k).all(|i| t[i] == t[k - i - 1]) {
        flag = false;
        break;
      }
    }
    if flag {
      ans += 1;
    }
  }
  println!("{ans}");
}
