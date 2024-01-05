#![allow(non_snake_case)]
use std::collections::HashSet;

use itertools::Itertools;
use proconio::{fastout, input, marker::Usize1};

#[fastout]
fn main() {
  input! {n:usize,_:usize}
  let p = (0..n).map(|_| {
    input! {p:u32,f:[Usize1]}
    (p, f.into_iter().collect::<HashSet<usize>>())
  });
  for p in p.permutations(2) {
    if p[0].0 <= p[1].0 && p[0].1.is_superset(&p[1].1) && p[0] != p[1] {
      println!("Yes");
      return;
    }
  }
  println!("No");
}
