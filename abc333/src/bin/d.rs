#![allow(non_snake_case)]
use itertools::Itertools;
use proconio::{fastout, input, marker::Usize1};

fn dfs(now: usize, prev: usize, edges: &Vec<Vec<usize>>) -> usize {
  let mut ret = 1;
  for &next in &edges[now] {
    if next != prev {
      ret += dfs(next, now, edges);
    }
  }
  ret
}

#[fastout]
fn main() {
  input! {n:usize,uv:[(Usize1,Usize1);n-1]}

  let mut edges = vec![vec![]; n];
  for (u, v) in uv {
    edges[u].push(v);
    edges[v].push(u);
  }
  let leaves = edges[0].iter().map(|&u| dfs(u, 0, &edges)).collect_vec();
  println!(
    "{}",
    leaves.iter().sum::<usize>() - leaves.iter().max().unwrap() + 1
  );
}
