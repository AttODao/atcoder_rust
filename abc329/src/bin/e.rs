use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
  input! {n:usize,m:usize,s:Chars,t:Chars}

  let mut dp = vec![vec![false; m + 1]; n + 1];
  dp[0][0] = true;
  for i in 0..n {
    if i + m <= n {
      dp[i][0] = dp[i].iter().any(|dp| *dp);
    }
    if dp[i][m] {
      for j in 0..=m {
        dp[i][j] = true;
      }
    }
    for j in 0..m {
      dp[i + 1][j + 1] = dp[i][j] && (s[i] == t[j]);
    }
  }
  if dp[n][m] {
    println!("Yes");
  } else {
    println!("No");
  }
}
