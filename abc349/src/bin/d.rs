#![allow(non_snake_case)]
use itertools::Itertools;
use proconio::{fastout, input};

#[fastout]
fn main() {
  input! {mut l:usize,r:usize}
  let mut ans = vec![];
  while l < r {
    for i in (0..=l.trailing_zeros().min(60)).rev() {
      let nl = l + (1 << i);
      if nl <= r {
        ans.push(format!("{} {}", l, nl));
        l = nl;
        break;
      }
    }
  }
  println!("{}\n{}", ans.len(), ans.iter().join("\n"));
}
