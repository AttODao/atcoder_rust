use itertools::{iproduct, izip, Itertools};
use petgraph::{algo::dijkstra, prelude::DiGraph};
use proconio::{fastout, input};

#[fastout]
fn main() {
  input! {n:usize,a:usize,b:usize,c:usize,d:[[usize;n];n]}

  let mut graph = DiGraph::<(), usize>::new();
  let cnodes = (0..n).map(|_| graph.add_node(())).collect_vec();
  let tnodes = (0..n).map(|_| graph.add_node(())).collect_vec();
  for (&cn, &tn) in izip!(&cnodes, &tnodes) {
    graph.add_edge(cn, tn, 0);
  }
  for (i, j) in iproduct!(0..n, 0..n) {
    graph.add_edge(cnodes[i], cnodes[j], d[i][j] * a);
    graph.add_edge(tnodes[i], tnodes[j], d[i][j] * b + c);
  }
  let ans = dijkstra(&graph, cnodes[0], Some(tnodes[n - 1]), |e| *e.weight())[&tnodes[n - 1]];
  println!("{}", ans);
}
