use std::cmp::max;

use proconio::{fastout, input};

#[fastout]
fn main() {
  input! {n:usize,m:usize,mut a:[usize;n]}

  a.sort();
  a.push(usize::MAX);
  let mut r = 0;
  let mut ans = 0;
  for l in 0..n {
    while a[r] < a[l] + m {
      r += 1;
    }
    ans = max(ans, r - l);
  }
  println!("{}", ans);
}
