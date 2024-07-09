#![allow(non_snake_case)]
use proconio::{fastout, input};

const S: [[usize; 3]; 5] = [[0, 0, 0], [0, 2, 3], [0, 3, 6], [0, 3, 7], [0, 4, 8]];

fn f(x: usize, y: usize) -> usize {
  ((x / 4) * (y / 2) * S[4][2] + (x / 4) * S[4][y % 2] + (y / 2) * S[x % 4][2] + S[x % 4][y % 2])
}

#[fastout]
fn main() {
  input! {mut a:i64,mut b:i64,mut c:i64,mut d:i64};
  let a = (a + (1 << 30)) as usize;
  let b = (b + (1 << 30)) as usize;
  let c = (c + (1 << 30)) as usize;
  let d = (d + (1 << 30)) as usize;
  println!("{}", f(a, b) + f(c, d) - f(a, d) - f(c, b));
}
