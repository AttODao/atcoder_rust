#![allow(non_snake_case)]
use std::cmp::Reverse;

use itertools::iproduct;
use proconio::{fastout, input};

#[fastout]
fn main() {
  input! {n:usize,mut fs:[(usize,usize);n]}

  let (i1, &(f1, s1)) = fs.iter().enumerate().max_by_key(|(_, (_, s))| s).unwrap();
  let mut ans = 0;

  for (i2, &(f2, s2)) in fs.iter().enumerate() {
    if i1 != i2 {
      ans = ans.max(if f1 == f2 { s1 + s2 / 2 } else { s1 + s2 });
    }
  }
  println!("{}", ans);
}
