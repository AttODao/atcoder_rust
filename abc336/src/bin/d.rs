#![allow(non_snake_case)]
use std::usize;

use proconio::{fastout, input};

#[fastout]
fn main() {
  input! {n:usize,a:[usize;n]}
  let mut minl = vec![0; n + 1];
  for i in 0..n {
    minl[i + 1] = a[i].min(minl[i] + 1);
  }
  let mut minr = vec![0; n + 1];
  for i in (0..n).rev() {
    minr[i] = a[i].min(minr[i + 1] + 1);
  }
  let mut ans = 0;
  for i in 0..n {
    ans = ans.max(minl[i + 1].min(minr[i]));
  }
  println!("{}", ans);
}
