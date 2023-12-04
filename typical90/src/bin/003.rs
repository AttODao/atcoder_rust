use itertools::Itertools;
use petgraph::{algo::dijkstra, graph::UnGraph, visit::Bfs};
use proconio::{fastout, input, marker::Usize1};

#[fastout]
fn main() {
  input! {n:usize,ab:[(Usize1,Usize1);n-1]}

  let mut graph = UnGraph::<(), ()>::new_undirected();
  let nodes = (0..n).map(|_| graph.add_node(())).collect_vec();
  graph.extend_with_edges(ab.into_iter().map(|(a, b)| (nodes[a], nodes[b])));
  let mut bfs = Bfs::new(&graph, nodes[0]);
  let mut last = nodes[0];
  while let Some(now) = bfs.next(&graph) {
    last = now;
  }
  let dist = dijkstra(&graph, last, None, |_| 1);
  println!("{}", dist.values().max().unwrap() + 1);
}
