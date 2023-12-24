#![allow(non_snake_case)]
use itertools::Itertools;
use maplit::hashset;
use proconio::{fastout, input, marker::Usize1};

#[fastout]
fn main() {
  input! {n:usize,a:[Usize1;n]}
  let mut discovered = hashset! {};
  let mut now = 0;
  for i in 0..n {
    now = a[now];
  }
  let mut ans = vec![];
  while !discovered.contains(&now) {
    ans.push(now + 1);
    discovered.insert(now);
    now = a[now];
  }
  println!("{}\n{}", ans.len(), ans.iter().join(" "));
}
