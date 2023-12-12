#![allow(non_snake_case)]
use itertools::iproduct;
use proconio::{fastout, input};

#[fastout]
fn main() {
  input! {n:usize,abcd:[(usize,usize,usize,usize);n]}

  let mut imos = [[0; 101]; 101];
  for (a, b, c, d) in abcd {
    imos[a][c] += 1;
    imos[b][d] += 1;
    imos[a][d] -= 1;
    imos[b][c] -= 1;
  }
  let mut sum = [[0; 102]; 102];
  let mut ans = 0;
  for (i, j) in iproduct!(0..101, 0..101) {
    sum[i + 1][j + 1] = imos[i][j] + sum[i][j + 1] + sum[i + 1][j] - sum[i][j];
    if sum[i + 1][j + 1] > 0 {
      ans += 1;
    }
  }
  println!("{}", ans);
}
