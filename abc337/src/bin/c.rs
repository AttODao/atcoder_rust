#![allow(non_snake_case)]
use itertools::Itertools;
use proconio::{fastout, input};

#[fastout]
fn main() {
  input! {n:usize,a:[i32;n]}
  let mut i = ((n * (n + 1) / 2) as i32 - a.iter().sum::<i32>() - 2) as usize;
  let mut ans_rev = vec![i + 1];
  while a[i] > 0 {
    i = a[i] as usize - 1;
    ans_rev.push(i + 1);
  }
  println!("{}", ans_rev.into_iter().rev().join(" "));
}
