#![allow(non_snake_case)]
use proconio::{fastout, input};

#[fastout]
fn main() {
  input! {n:usize,m:usize,p:u64,a:[u64;n],mut b:[u64;m]}

  b.sort();
  let mut sum = vec![0; m + 1];
  for i in 0..m {
    sum[i + 1] = sum[i] + b[i];
  }
  let mut ans = 0;
  for a in a {
    let k = b.partition_point(|b| a + b < p);
    ans += k as u64 * a + sum[k] + (m - k) as u64 * p;
  }
  println!("{}", ans);
}
