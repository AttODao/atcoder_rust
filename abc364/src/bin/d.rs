#![allow(non_snake_case)]
use itertools::Itertools;
use proconio::{fastout, input};

const BIG: i64 = i64::MAX / 2;

#[fastout]
fn main() {
  input! {n:usize,q:usize,mut a:[i64;n],bk:[(i64,usize);q]}
  a.push(-BIG);
  a.push(BIG);
  a.sort_unstable();
  for (b, k) in bk {
    let i = (0..n - k + 3)
      .collect_vec()
      .partition_point(|&i| b - a[i] > a[i + k - 1] - b);
    println!("{}", (b - a[i - 1]).min(a[i + k - 1] - b));
  }
}
