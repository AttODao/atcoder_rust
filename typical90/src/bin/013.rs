use itertools::Itertools;
use petgraph::{algo::dijkstra, graph::UnGraph};
use proconio::{fastout, input, marker::Usize1};

#[fastout]
fn main() {
  input! {n:usize,m:usize,abc:[(Usize1,Usize1,i64);m]}

  let mut graph = UnGraph::<(), i64>::new_undirected();
  let nodes = (0..n).map(|_| graph.add_node(())).collect_vec();
  abc.iter().for_each(|&(a, b, c)| {
    graph.add_edge(nodes[a], nodes[b], c);
  });
  let mut dist1 = dijkstra(&graph, nodes[0], None, |e| *e.weight());
  dist1.insert(nodes[0], 0);
  let mut dist2 = dijkstra(&graph, nodes[n - 1], None, |e| *e.weight());
  dist2.insert(nodes[n - 1], 0);
  for k in 0..n {
    println!("{}", dist1[&nodes[k]] + dist2[&nodes[k]]);
  }
}
