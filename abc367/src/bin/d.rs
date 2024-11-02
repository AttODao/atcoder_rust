#![allow(non_snake_case)]
use proconio::{fastout, input};

#[fastout]
fn main() {
  input! {n:usize,m:usize,a:[usize;n]}
  let mut b = vec![0; 2 * n];
  for i in 0..2 * n - 1 {
    b[i + 1] = (b[i] + a[i % n]) % m;
  }
  let mut table = vec![0; m];
  let mut ans = 0usize;
  for i in 0..n {
    table[b[i]] += 1;
  }
  for i in 0..n {
    table[b[i]] -= 1;
    ans += table[b[i + n]];
    table[b[i + n]] += 1;
  }
  println!("{ans}")
}
