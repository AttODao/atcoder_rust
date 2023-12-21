#![allow(non_snake_case)]
use ac_library::ModInt998244353;
use itertools::Itertools;
use proconio::{fastout, input, marker::Chars};

type Mint = ModInt998244353;

#[fastout]
fn main() {
  input! {n:usize,s:Chars}
  for (&s1, &s2) in s.iter().tuple_windows() {
    if s1 != '1' && s2 != '1' {
      println!("-1");
      return;
    }
  }
  let mut ans: Mint = 0.into();
  for i in (1..n).rev() {
    ans += 1;
    ans *= s[i] as u64 - '0' as u64;
  }
  println!("{}", ans);
}
