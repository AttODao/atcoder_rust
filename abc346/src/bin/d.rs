#![allow(non_snake_case)]
use itertools::{izip, Itertools};
use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
  input! {n:usize,s:Chars,c:[usize;n]}
  let s = s.into_iter().map(|s| (s == '1') as usize).collect_vec();
  let mut dp = [[0; 2]; 2];
  dp[0][s[0] ^ 1] = c[0];
  dp[1] = [usize::MAX / 2; 2];
  for (s, c) in izip!(s, c).skip(1) {
    let mut ndp = dp.clone();
    for i in 0..2 {
      ndp[0][i] = dp[0][i ^ 1] + (s ^ i) * c;
      ndp[1][i] = dp[0][i].min(dp[1][i ^ 1]) + (s ^ i) * c;
    }
    dp = ndp;
  }
  println!("{}", dp[1][0].min(dp[1][1]));
}
