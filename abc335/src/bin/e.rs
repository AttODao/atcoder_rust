#![allow(non_snake_case)]
use std::{
  cmp::{Ordering, Reverse},
  collections::BinaryHeap,
  vec,
};

use proconio::{fastout, input, marker::Usize1};

#[fastout]
fn main() {
  input! {n:usize,m:usize,a:[usize;n],uv:[(Usize1,Usize1);m]}
  let mut edges = vec![vec![]; n];
  for (u, v) in uv {
    edges[u].push(v);
    edges[v].push(u);
  }
  let mut score = vec![0; n];
  let mut pq = BinaryHeap::new();
  score[0] = 1;
  pq.push((Reverse(a[0]), 1, 0));
  while let Some((Reverse(av), sv, v)) = pq.pop() {
    if sv >= score[v] {
      for &u in &edges[v] {
        let tempsu = match a[u].cmp(&av) {
          Ordering::Less => 0,
          Ordering::Equal => sv,
          Ordering::Greater => sv + 1,
        };
        if tempsu > score[u] {
          score[u] = tempsu;
          pq.push((Reverse(a[u]), tempsu, u));
        }
      }
    }
  }
  println!("{}", score[n - 1]);
}
