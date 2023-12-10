#![allow(non_snake_case)]
use itertools::Itertools;
use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
  input! {s:Chars}

  let mut ans = 1;
  for (l, r) in (0..s.len()).tuple_combinations() {
    let t = s[l..=r].into_iter().collect_vec();
    if t == t.iter().rev().map(|t| *t).collect_vec() {
      ans = ans.max(r - l + 1);
    }
  }
  println!("{}", ans);
}
