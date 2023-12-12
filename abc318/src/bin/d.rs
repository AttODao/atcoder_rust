#![allow(non_snake_case)]

use itertools::Itertools;
use proconio::{fastout, input};

#[fastout]
fn main() {
  input! {n:usize}
  let mut d = vec![vec![0; n]; n];
  for (i, j) in (0..n).tuple_combinations() {
    input! {nd:usize}
    d[i][j] = nd;
  }

  let mut dp = vec![0; 1 << n];
  for s in 0..(1 << n) - 1 {
    let mut zero = (0..n).filter(|i| s >> i & 1 == 0);
    let a = zero.next().unwrap();
    for b in zero {
      let ns = s | (1 << a) | (1 << b);
      dp[ns] = dp[ns].max(dp[s] + d[a][b]);
    }
  }
  println!("{}", dp.last().unwrap());
}
