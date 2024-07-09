#![allow(non_snake_case)]
use itertools::Itertools;
use proconio::{fastout, input};

#[fastout]
fn main() {
  input! {n:usize,l:usize,r:usize}
  println!("{}", (1..l).chain((l..=r).rev()).chain(r + 1..=n).join(" "));
}
