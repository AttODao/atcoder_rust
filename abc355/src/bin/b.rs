#![allow(non_snake_case)]
use itertools::Itertools;
use proconio::{fastout, input};

#[fastout]
fn main() {
  input! {n:usize,m:usize,a:[usize;n],b:[usize;m]}
  if a
    .into_iter()
    .map(|a| (a, true))
    .chain(b.into_iter().map(|b| (b, false)))
    .sorted()
    .tuple_windows()
    .any(|((_, p), (_, q))| p && q)
  {
    println!("Yes");
  } else {
    println!("No");
  }
}
