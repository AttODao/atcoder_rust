#![allow(non_snake_case)]
use itertools::iproduct;
use proconio::{fastout, input};

#[fastout]
fn main() {
  input! {h:usize,w:usize,ab:[(usize,usize)]}
  let mut sum = vec![vec![0; w + 1]; h + 1];
  for (a, b) in ab {
    sum[a][b] = 1;
  }
  for (i, j) in iproduct!(0..h, 0..w) {
    sum[i + 1][j + 1] += sum[i + 1][j] + sum[i][j + 1] - sum[i][j];
  }
  let mut ans = 0;
  for (i, j) in iproduct!(0..h, 0..w) {
    let (mut ok, mut ng) = (0, (h - i).min(w - j) + 1);
    while ng - ok > 1 {
      let mid = (ok + ng) / 2;
      if sum[i][j] + sum[i + mid][j + mid] - sum[i][j + mid] - sum[i + mid][j] == 0 {
        ok = mid;
      } else {
        ng = mid;
      }
    }
    ans += ok;
  }
  println!("{}", ans);
}
