#![allow(non_snake_case)]
use amplify::confinement::Collection;
use itertools::Itertools;
use proconio::{fastout, input};
use std::collections::BTreeSet;

#[fastout]
fn main() {
  input! {ac:[(usize,usize)]}
  let mut p = usize::MAX;
  let mut ans = BTreeSet::new();
  for (_, c, i) in ac
    .into_iter()
    .enumerate()
    .map(|(i, (a, c))| (a, c, i))
    .sorted()
    .rev()
  {
    if c <= p {
      p = c;
      ans.push(i + 1);
    }
  }
  println!("{}\n{}", ans.len(), ans.iter().join(" "));
}
