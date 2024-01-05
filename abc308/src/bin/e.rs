#![allow(non_snake_case)]
use itertools::izip;
use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
  input! {n:usize,a:[usize;n],s:Chars}
  let mut dp = vec![vec![0; 8]; 4];
  dp[0][0] = 1;
  for (a, s) in izip!(a, s) {
    let i = "MEX".chars().enumerate().find(|&(i, d)| s == d).unwrap().0;
    for j in 0..8 {
      dp[i + 1][j | 1 << a] += dp[i][j];
    }
  }
  println!(
    "{}",
    dp[3]
      .iter()
      .enumerate()
      .map(|(i, dp)| (0..4).find(|j| i >> j & 1 == 0).unwrap() * dp)
      .sum::<usize>()
  )
}
