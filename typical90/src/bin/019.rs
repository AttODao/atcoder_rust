use proconio::{fastout, input};

#[fastout]
fn main() {
  input! {n:usize,a:[i32;2*n]}

  let mut dp = vec![vec![i32::MAX; 2 * n]; 2 * n];
  for i in 0..2 * n - 1 {
    dp[i][i + 1] = (a[i] - a[i + 1]).abs();
  }
  for (l, r) in (1..n).flat_map(|d| (0..2 * n - 2 * d - 1).map(move |l: usize| (l, l + 2 * d + 1)))
  {
    dp[l][r] = dp[l + 1][r - 1] + (a[l] - a[r]).abs();
    for k in (l + 1..r - 1).filter(|k| (k - l) & 1 > 0) {
      dp[l][r] = dp[l][r].min(dp[l][k] + dp[k + 1][r])
    }
  }
  println!("{}", dp[0][2 * n - 1]);
}
