#![allow(non_snake_case)]
use itertools::iproduct;
use proconio::{fastout, input};

#[fastout]
fn main() {
  input! {mut n:usize}
  let mut dp = [[[[[0; 2]; 127]; 127]; 127]; 16];
  for s in 1..=126 {
    dp[0][s][0][0][0] = 1;
  }
  let mut pow10 = 1;
  for d in 0..15 {
    let nd = n % 10;
    for (s, i, f, t) in iproduct!(1..=126, 0..=126, 0..2, 0..10) {
      for j in 0..s {
        if i + t > s {
          break;
        }
        dp[d + 1][s][i + t][(pow10 * t + j) % s][(2 * nd < 2 * t + f) as usize] +=
          dp[d][s][i][j][f];
      }
    }
    n /= 10;
    pow10 *= 10;
  }
  println!("{}", (1..=126).map(|s| dp[15][s][s][0][0]).sum::<usize>());
}
