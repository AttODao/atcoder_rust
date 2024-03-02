#![allow(non_snake_case)]
use std::collections::VecDeque;

use itertools::Itertools;
use proconio::{fastout, input, marker::Usize1};

#[fastout]
fn main() {
  input! {n:usize,uv:[(Usize1,Usize1);n-1]}
  let mut edges = vec![vec![]; n];
  for (u, v) in uv {
    edges[u].push(v);
    edges[v].push(u);
  }
  let (mut now, mut prev) = (0, usize::MAX);
  while edges[now].len() > 1 {
    for &next in &edges[now] {
      if next != prev {
        prev = now;
        now = next;
        break;
      }
    }
  }
  let mut bfs = VecDeque::new();
  let mut ans = vec![];
  bfs.push_back((0, edges[now][0], usize::MAX));
  while let Some((mut num, now, prev)) = bfs.pop_front() {
    if num == 0 {
      ans.push(edges[now].len())
    }
    num = (num + 1) % 3;
    for &next in &edges[now] {
      if next != prev {
        bfs.push_back((num, next, now));
      }
    }
  }
  println!("{}", ans.into_iter().sorted().join(" "));
}
