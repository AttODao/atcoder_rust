#![allow(non_snake_case)]
use std::collections::BinaryHeap;

use itertools::Itertools;
use proconio::{fastout, input, marker::Usize1};

#[fastout]
fn main() {
  input! {n:usize,m:usize,k:usize,ab:[(Usize1,Usize1);m],ph:[(Usize1,i32);k]}
  let mut edges = vec![vec![]; n];
  for (a, b) in ab {
    edges[a].push(b);
    edges[b].push(a);
  }
  let mut bfs = ph
    .into_iter()
    .map(|(p, h)| (h, p))
    .collect::<BinaryHeap<_>>();
  let mut discovered = vec![false; n];
  while let Some((h, now)) = bfs.pop() {
    if discovered[now] {
      continue;
    }
    discovered[now] = true;
    if h > 0 {
      for &next in &edges[now] {
        if !discovered[next] {
          bfs.push((h - 1, next));
        }
      }
    }
  }
  let ans = (1..=n).filter(|i| discovered[i - 1]).collect_vec();
  println!("{}\n{}", ans.len(), ans.iter().join(" "));
}
