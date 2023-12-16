#![allow(non_snake_case)]
use std::collections::VecDeque;

use ac_library::SccGraph;
use itertools::Itertools;
use proconio::{fastout, input, marker::Usize1};

#[fastout]
fn main() {
  input! {n:usize}
  let mut p = vec![];
  for _ in 0..n {
    input! {c:usize,np:[Usize1;c]}
    p.push(np);
  }
  let mut bfs = VecDeque::new();
  let mut discovered = vec![false; n];
  let mut graph = SccGraph::new(n);
  bfs.push_back(0);
  while let Some(now) = bfs.pop_front() {
    for &next in &p[now] {
      graph.add_edge(now, next);
      if !discovered[next] {
        bfs.push_back(next);
        discovered[next] = true;
      }
    }
  }
  println!(
    "{}",
    graph
      .scc()
      .iter()
      .filter_map(|s| {
        let s = s[0];
        if discovered[s] {
          Some(s + 1)
        } else {
          None
        }
      })
      .rev()
      .join(" ")
  )
}
