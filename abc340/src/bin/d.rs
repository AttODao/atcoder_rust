#![allow(non_snake_case)]
use itertools::Itertools;
use petgraph::{algo::dijkstra, graph::DiGraph};
use proconio::{fastout, input, marker::Usize1};

#[fastout]
fn main() {
  input! {n:usize,abx:[(u64,u64,Usize1);n-1]}
  let mut graph = DiGraph::<(), u64>::new();
  let nodes = (0..n).map(|_| graph.add_node(())).collect_vec();
  for (i, (a, b, x)) in abx.into_iter().enumerate() {
    graph.add_edge(nodes[i], nodes[i + 1], a);
    graph.add_edge(nodes[i], nodes[x], b);
  }
  let res = dijkstra(&graph, nodes[0], Some(nodes[n - 1]), |e| *e.weight());
  println!("{}", res[&nodes[n - 1]]);
}
