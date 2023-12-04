use num_traits::Float;
use proconio::{fastout, input};

#[fastout]
fn main() {
  input! {n:usize,p:[f64;n]}

  let mut dp = vec![0.0; n + 1];

  for (i, p) in p.iter().enumerate() {
    for j in (0..(i + 1)).rev() {
      dp[j + 1] = dp[j + 1].max(p + dp[j] * 0.9);
    }
  }
  let mut ans = -1200.0;
  let mut m = 0.0;
  for i in 1..=n {
    m = m * 0.9 + 1.0;
    let perf = dp[i] / m - 1200.0 / (i as f64).sqrt();
    ans = ans.max(perf);
  }
  println!("{:.12}", ans);
}
