#![allow(non_snake_case)]
use proconio::{fastout, input};

#[fastout]
fn main() {
  input! {t:String,s:[[String]]}
  let mut dp = vec![u32::MAX / 2; t.len() + 1];
  dp[0] = 0;
  for s in s {
    for l in (0..t.len()).rev() {
      for s in &s {
        if t[l..].starts_with(s) {
          dp[l + s.len()] = dp[l + s.len()].min(dp[l] + 1);
        }
      }
    }
  }
  println!(
    "{}",
    if dp[t.len()] < u32::MAX / 2 {
      dp[t.len()].to_string()
    } else {
      "-1".to_string()
    }
  )
}
