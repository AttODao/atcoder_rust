#![allow(non_snake_case)]
use itertools::iproduct;
use proconio::{fastout, input};

#[fastout]
fn main() {
  input! {a:usize,b:usize,c:usize}
  let mut dp = [[None; 61]; 61];
  dp[0][0] = Some((0, 0));
  for k in 0..60 {
    let mut ndp = [[None; 61]; 61];
    for (i, j) in iproduct!(0..60, 0..60) {
      if let Some((x, y)) = dp[i][j] {
        for (dx, dy) in iproduct!(0..2, 0..2) {
          if dx ^ dy == c >> k & 1 {
            ndp[i + dx][j + dy] = Some((x | (dx << k), y | (dy << k)));
          }
        }
      }
    }
    dp = ndp;
  }
  if let Some((x, y)) = dp[a][b] {
    println!("{} {}", x, y);
  } else {
    println!("-1");
  }
}
