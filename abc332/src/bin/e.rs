#![allow(non_snake_case)]

use itertools::Itertools;
use proconio::{fastout, input};

#[fastout]
fn main() {
  input! {n:usize,d:usize,w:[usize;n]}

  let w2t = (0..1 << n)
    .map(|i| {
      (0..n)
        .filter_map(|j| if (i >> j) & 1 > 0 { Some(w[j]) } else { None })
        .sum::<usize>()
        .pow(2)
    })
    .collect_vec();
  let mut dp = w2t.clone();
  for _ in 1..d {
    let mut ndp = vec![usize::MAX / 2; 1 << n];
    for s in 0..1 << n {
      let mut t = s;
      while t > 0 {
        ndp[s] = ndp[s].min(dp[s ^ t] + w2t[t]);
        t = (t - 1) & s;
      }
      ndp[s] = ndp[s].min(dp[s ^ t] + w2t[t]);
    }
    dp = ndp;
  }

  println!(
    "{}",
    (d * dp[(1 << n) - 1] - w.iter().sum::<usize>().pow(2)) as f64 / (d * d) as f64
  );
}
