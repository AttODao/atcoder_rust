#![allow(non_snake_case)]
use proconio::{fastout, input, marker::Usize1};

#[fastout]
fn main() {
  input! {n:usize,a:[Usize1]}
  let mut r = vec![n; n];
  let mut c = vec![n; n];
  let (mut d1, mut d2) = (n, n);
  for (k, a) in a.into_iter().enumerate() {
    let (i, j) = (a / n, a % n);
    r[i] -= 1;
    c[j] -= 1;
    if i == j {
      d1 -= 1;
    }
    if i + j == n - 1 {
      d2 -= 1;
    }
    if r[i] == 0 || c[j] == 0 || d1 == 0 || d2 == 0 {
      println!("{}", k + 1);
      return;
    }
  }
  println!("-1");
}
