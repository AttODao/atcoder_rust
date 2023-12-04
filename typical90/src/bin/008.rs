use ac_library::ModInt1000000007;
use itertools::Itertools;
use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
  input! {_:usize,s:Chars}

  let target = "atcoder".chars().collect_vec();
  let mut dp = vec![ModInt1000000007::new(0); target.len() + 1];
  dp[0] += 1;
  for c in s {
    for (i, &t) in target.iter().enumerate() {
      if c == t {
        let d = dp[i];
        dp[i + 1] += d;
        break;
      }
    }
  }
  println!("{}", dp[target.len()]);
}
