#![allow(non_snake_case)]
use itertools::Itertools;
use petgraph::{algo::dijkstra, graph::UnGraph};
use proconio::{fastout, input, marker::Usize1};

#[fastout]
fn main() {
  input! {n1:usize,n2:usize,ab:[(Usize1,Usize1)]}
  let mut graph1 = UnGraph::<(), ()>::new_undirected();
  let mut graph2 = UnGraph::<(), ()>::new_undirected();
  let nodes1 = (0..n1).map(|_| graph1.add_node(())).collect_vec();
  let nodes2 = (0..n2).map(|_| graph2.add_node(())).collect_vec();
  for (a, b) in ab {
    if a < n1 {
      graph1.add_edge(nodes1[a], nodes1[b], ());
    } else {
      graph2.add_edge(nodes2[a - n1], nodes2[b - n1], ());
    }
  }
  let d1 = dijkstra(&graph1, nodes1[0], None, |_| 1)
    .into_iter()
    .map(|(_, d)| d)
    .max()
    .unwrap();
  let d2 = dijkstra(&graph2, nodes2[n2 - 1], None, |_| 1)
    .into_iter()
    .map(|(_, d)| d)
    .max()
    .unwrap();
  println!("{}", d1 + d2 + 1);
}
