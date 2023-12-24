#![allow(non_snake_case)]
use proconio::{fastout, input, marker::Usize1};

#[fastout]
fn main() {
  input! {n:usize,k:usize,a:[Usize1;k]}

  let mut socks = vec![];
  let mut kindex = 0;
  for i in 0..n {
    if kindex < k && a[kindex] == i {
      socks.push(i);
      kindex += 1;
    } else {
      socks.append(&mut vec![i, i]);
    }
  }
  let len = socks.len();
  let mut dp = vec![usize::MAX; len + 1];
  dp[0] = 0;
  for i in 0..len - 1 {
    if i & 1 == 0 {
      dp[i + 2] = dp[i + 2].min(dp[i] + socks[i].abs_diff(socks[i + 1]));
      dp[i + 1] = dp[i + 1].min(dp[i]);
    } else {
      dp[i + 2] = dp[i + 2].min(dp[i] + socks[i].abs_diff(socks[i + 1]));
    }
  }
  if (len - 1) & 1 == 0 {
    dp[len] = dp[len].min(dp[len - 1]);
  }
  println!("{}", dp[len]);
}
