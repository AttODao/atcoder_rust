#![allow(non_snake_case)]
use std::iter::once;

use ac_library::ModInt998244353;
use itertools::{izip, Itertools};
use proconio::{fastout, input, marker::Chars};

type Mint = ModInt998244353;

#[fastout]
fn main() {
  input! {s:Chars}
  let n = s.len();
  let mut dp = vec![Mint::new(0); n + 1];
  dp[0] += 1;
  for s in s {
    match s {
      '(' => {
        dp.rotate_right(1);
        dp[0] = 0.into();
      }
      ')' => {
        dp.rotate_left(1);
        dp[n] = 0.into();
      }
      '?' => {
        dp = izip!(
          once(&Mint::new(0)).chain(dp.iter()).take(n + 1),
          dp.iter().skip(1).chain(once(&Mint::new(0)))
        )
        .map(|(r, l)| r + l)
        .collect_vec();
      }
      _ => (),
    }
  }
  println!("{}", dp[0]);
}
