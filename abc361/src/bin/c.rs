#![allow(non_snake_case)]
use itertools::izip;
use proconio::{fastout, input};

#[fastout]
fn main() {
  input! {n:usize,k:usize,mut a:[u32;n]}
  a.sort();
  println!("{}", izip!(&a[0..=k], &a[n-k-1..n]).map(|(min, max)|max-min).min().unwrap());
}
