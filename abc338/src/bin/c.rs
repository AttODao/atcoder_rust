#![allow(non_snake_case)]
use itertools::izip;
use proconio::{fastout, input};

#[fastout]
fn main() {
  input! {n:usize,q:[i32;n],a:[i32;n],b:[i32;n]}
  let mut ans = 0;
  for i in 0..=izip!(&q, &a)
    .filter(|(_, &a)| a > 0)
    .map(|(&q, &a)| q / a)
    .min()
    .unwrap()
  {
    let j = izip!(&q, &a, &b)
      .filter(|(_, _, &b)| b > 0)
      .map(|(&q, &a, &b)| (q - a * i) / b)
      .min()
      .unwrap();
    ans = ans.max(i + j);
  }
  println!("{}", ans);
}
