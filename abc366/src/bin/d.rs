#![allow(non_snake_case)]
use itertools::iproduct;
use proconio::{fastout, input, marker::Usize1};

#[fastout]
fn main() {
  input! {n:usize,a:[[[i64;n];n];n],lr:[(Usize1,usize,Usize1,usize,Usize1,usize)]}
  let mut s = vec![vec![vec![0; n + 1]; n + 1]; n + 1];
  for (i, j, k) in iproduct!(0..n, 0..n, 0..n) {
    s[i + 1][j + 1][k + 1] = a[i][j][k]
      + (s[i][j + 1][k + 1] + s[i + 1][j][k + 1] + s[i + 1][j + 1][k])
      - (s[i + 1][j][k] + s[i][j + 1][k] + s[i][j][k + 1])
      + s[i][j][k];
  }
  for (lx, rx, ly, ry, lz, rz) in lr {
    println!(
      "{}",
      s[rx][ry][rz] - (s[lx][ry][rz] + s[rx][ly][rz] + s[rx][ry][lz])
        + (s[rx][ly][lz] + s[lx][ry][lz] + s[lx][ly][rz])
        - s[lx][ly][lz]
    );
  }
}
