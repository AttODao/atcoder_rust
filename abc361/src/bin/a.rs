#![allow(non_snake_case)]
use itertools::Itertools;
use proconio::{input, fastout};

#[fastout]
fn main() {
  input! {n:usize,k:usize,x:u32,mut a:[u32;n]}
  a.insert(k, x);
  println!("{}", a.iter().join(" "));
}
