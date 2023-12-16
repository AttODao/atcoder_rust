#![allow(non_snake_case)]
use proconio::{fastout, input};

#[fastout]
fn main() {
  input! {n:usize,a:[usize;n]}

  let min = *a.iter().min().unwrap();
  let mut table = vec![true; n + 1];
  for a in a {
    table[a - min] = false;
  }
  println!("{}", table.iter().position(|t| *t).unwrap() + min)
}
