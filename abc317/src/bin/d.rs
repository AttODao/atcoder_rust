#![allow(non_snake_case)]
use proconio::{fastout, input};

#[fastout]
fn main() {
  input! {n:usize,xyz:[(usize,usize,usize);n]}

  let sumz = xyz.iter().map(|(_, _, z)| z).sum::<usize>();
  let mut dp = vec![usize::MAX / 2; sumz + 1];
  dp[0] = 0;
  for (x, y, z) in xyz {
    if x > y {
      for i in (0..=sumz).rev() {
        dp[i] = if i < z { usize::MAX / 2 } else { dp[i - z] }
      }
    } else {
      for i in (z..=sumz).rev() {
        dp[i] = dp[i].min(dp[i - z] + (y - x + 1) / 2)
      }
    }
  }
  println!("{}", dp[(sumz + 1) / 2..].iter().min().unwrap());
}
