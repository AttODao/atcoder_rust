#![allow(non_snake_case)]
use itertools::Itertools;
use proconio::{fastout, input};

#[fastout]
fn main() {
  input! {n:usize,k:usize,a:[usize;n]}
  println!("{}", a[n - k..n].iter().chain(a[0..n - k].iter()).join(" "));
}
