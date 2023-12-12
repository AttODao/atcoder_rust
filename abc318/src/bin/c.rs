#![allow(non_snake_case)]
use std::cmp::Reverse;

use proconio::{fastout, input};

#[fastout]
fn main() {
  input! {n:usize,d:usize,p:usize,mut f:[usize;n]}

  f.sort_by_key(|&f| Reverse(f));
  let mut ans = 0;
  for l in (0..n).step_by(d) {
    let r = f.len().min(l + d);
    ans += f[l..r].iter().sum::<usize>().min(p);
  }
  println!("{}", ans);
}
