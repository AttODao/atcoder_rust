#![allow(non_snake_case)]
use proconio::{fastout, input};

#[fastout]
fn main() {
  input! {n:usize,k:usize,p:usize,ca:[(i64,[usize;k]);n]}

  let mut powp1 = vec![1; k + 1];
  for i in 0..k {
    powp1[i + 1] = powp1[i] * (p + 1);
  }
  let mut dp = vec![i64::MAX / 2; powp1[k]];
  dp[0] = 0;
  for (c, a) in ca {
    for i in (0..powp1[k]).rev() {
      let next = (0..k)
        .map(|j| p.min(i / powp1[j] % (p + 1) + a[j]) * powp1[j])
        .sum::<usize>();
      dp[next] = dp[next].min(dp[i] + c);
    }
  }
  println!(
    "{}",
    if dp[powp1[k] - 1] == i64::MAX / 2 {
      -1
    } else {
      dp[powp1[k] - 1]
    }
  );
}
