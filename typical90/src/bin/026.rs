use itertools::Itertools;
use petgraph::{graph::UnGraph, visit::Dfs};
use proconio::{fastout, input, marker::Usize1};

#[fastout]
fn main() {
  input! {n:usize,ab:[(Usize1,Usize1);n-1]}

  let mut graph = UnGraph::<bool, ()>::new_undirected();
  let nodes = (0..n).map(|_| graph.add_node(false)).collect_vec();
  for (a, b) in ab {
    graph.add_edge(nodes[a], nodes[b], ());
  }
  let mut dfs = Dfs::new(&graph, nodes[0]);
  while let Some(now) = dfs.next(&graph) {
    let mut nei = graph.neighbors(now).detach();
    while let Some((_, next)) = nei.next(&graph) {
      graph[next] = !graph[now];
    }
  }
  let flag = graph.node_indices().filter(|&i| graph[i]).count() >= n / 2;
  println!(
    "{}",
    (1..=n)
      .filter(|&i| graph[nodes[i - 1]] == flag)
      .take(n / 2)
      .join(" ")
  );
}
