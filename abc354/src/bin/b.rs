#![allow(non_snake_case)]
use proconio::{fastout, input};

#[fastout]
fn main() {
  input! {n:usize,sc:[(String,usize);n]}
  let (mut s, c): (Vec<_>, Vec<_>) = sc.into_iter().unzip();
  s.sort();
  println!("{}", s[c.iter().sum::<usize>() % n]);
}
