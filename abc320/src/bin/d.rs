#![allow(non_snake_case)]

use itertools::Itertools;
use petgraph::{
  graph::DiGraph,
  visit::{Dfs, VisitMap},
};
use proconio::{fastout, input, marker::Usize1};

fn add(a: (i64, i64), b: (i64, i64)) -> (i64, i64) {
  (a.0 + b.0, a.1 + b.1)
}

#[fastout]
fn main() {
  input! {n:usize,m:usize,abxy:[(Usize1,Usize1,i64,i64);m]}

  let mut graph = DiGraph::new();
  let nodes = (0..n).map(|_| graph.add_node((0, 0))).collect_vec();
  for (a, b, x, y) in abxy {
    graph.add_edge(nodes[a], nodes[b], (x, y));
    graph.add_edge(nodes[b], nodes[a], (-x, -y));
  }
  let mut dfs = Dfs::new(&graph, nodes[0]);
  while let Some(now) = dfs.next(&graph) {
    let mut nes = graph.neighbors(now).detach();
    while let Some((edge, next)) = nes.next(&graph) {
      graph[next] = add(graph[now], graph[edge]);
    }
  }
  println!(
    "{}",
    nodes
      .iter()
      .map(|&node| if dfs.discovered.is_visited(&node) {
        format!("{} {}", graph[node].0, graph[node].1)
      } else {
        "undecidable".to_string()
      })
      .collect_vec()
      .join("\n")
  );
}
