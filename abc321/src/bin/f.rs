#![allow(non_snake_case)]
use ac_library::ModInt998244353;
use proconio::{fastout, input, marker::Usize1};

#[fastout]
fn main() {
  input! {q:usize,k:usize,tx:[(char,usize);q]}

  let mut dp = vec![ModInt998244353::new(0); k + 1];
  dp[0] += 1;
  for (t, x) in tx {
    if t == '+' {
      for i in (x..=k).rev() {
        let add = dp[i - x];
        dp[i] += add;
      }
    } else {
      for i in x..=k {
        let sub = dp[i - x];
        dp[i] -= sub;
      }
    }
    println!("{}", dp[k]);
  }
}
