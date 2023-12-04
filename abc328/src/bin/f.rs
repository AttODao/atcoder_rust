use ac_library::Dsu;
use itertools::Itertools;
use petgraph::{graph::DiGraph, visit::Bfs};
use proconio::{fastout, input, marker::Usize1};

#[fastout]
fn main() {
  input! {n:usize,q:usize,abd:[(Usize1,Usize1,i64);q]}

  let mut uf = Dsu::new(n);
  let mut graph: DiGraph<i64, i64> = DiGraph::new();
  let nodes = (0..n).map(|_| graph.add_node(0)).collect_vec();
  for &(a, b, d) in abd.iter() {
    if !uf.same(a, b) {
      uf.merge(a, b);
      graph.add_edge(nodes[a], nodes[b], -d);
      graph.add_edge(nodes[b], nodes[a], d);
    }
  }
  for a in uf.groups() {
    let mut bfs = Bfs::new(&graph, nodes[a[0]]);
    while let Some(now) = bfs.next(&graph) {
      let mut nei = graph.neighbors(now).detach();
      while let Some((e, next)) = nei.next(&graph) {
        graph[next] = graph[now] + graph[e];
      }
    }
  }
  for (i, &(a, b, d)) in abd.iter().enumerate() {
    if graph[nodes[a]] - graph[nodes[b]] == d {
      print!("{} ", i + 1);
    }
  }
}
