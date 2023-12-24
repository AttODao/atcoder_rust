#![allow(non_snake_case)]

use itertools::iproduct;
use proconio::{fastout, input, marker::Chars};

fn f(i: usize, j: usize, n: usize, sum: &Vec<Vec<usize>>) -> usize {
  let (X, x, Y, y) = (i / n, i % n, j / n, j % n);
  sum[n][n] * X * Y + sum[x][y] + sum[x][n] * Y + sum[n][y] * X
}

#[fastout]
fn main() {
  input! {n:usize,q:usize,p:[Chars;n],abcd:[(usize,usize,usize,usize);q]}

  let mut sum = vec![vec![0; n + 1]; n + 1];
  for (i, j) in iproduct!(0..n, 0..n) {
    sum[i + 1][j + 1] = sum[i][j + 1] + sum[i + 1][j] - sum[i][j];
    if p[i][j] == 'B' {
      sum[i + 1][j + 1] += 1;
    }
  }
  for (a, b, c, d) in abcd {
    println!(
      "{}",
      f(a, b, n, &sum) + f(c + 1, d + 1, n, &sum) - f(a, d + 1, n, &sum) - f(c + 1, b, n, &sum)
    );
  }
}
