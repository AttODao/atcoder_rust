use proconio::{fastout, input};

#[fastout]
fn main() {
  input! {n:usize,mut dcs:[(usize,usize,i64);n]}

  dcs.sort();
  let maxd = dcs[dcs.len() - 1].0;
  let mut dp = vec![0; maxd + 1];
  for (d, c, s) in dcs {
    for i in (c..=d).rev() {
      dp[i] = dp[i].max(dp[i - c] + s);
    }
  }
  println!("{}", dp.iter().max().unwrap());
}
