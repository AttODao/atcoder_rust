#![allow(non_snake_case)]
use itertools::iproduct;
use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
  input! {h:usize,w:usize,n:usize,t:Chars,s:[Chars;h]}
  let mut dp = vec![vec![vec![false; w]; h]; n + 1];
  for (i, j) in iproduct!(0..h, 0..w) {
    dp[0][i][j] = s[i][j] == '.';
  }
  for k in 0..n {
    let (di, dj) = match t[k] {
      'L' => (0, -1),
      'R' => (0, 1),
      'U' => (-1, 0),
      'D' => (1, 0),
      _ => unreachable!(),
    };
    for (i, j) in iproduct!(0..h, 0..w) {
      dp[k + 1][i][j] =
        dp[0][i][j] && dp[k][i.saturating_add_signed(-di)][j.saturating_add_signed(-dj)]
    }
  }
  println!(
    "{}",
    dp[n]
      .iter()
      .map(|dp| dp.iter().map(|&dp| dp as u32).sum::<u32>())
      .sum::<u32>()
  );
}
