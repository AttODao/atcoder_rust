use ac_library::SccGraph;
use proconio::{fastout, input, marker::Usize1};

#[fastout]
fn main() {
  input! {n:usize,m:usize,ab:[(Usize1,Usize1);m]}

  let mut graph = SccGraph::new(n);
  ab.into_iter().for_each(|(a, b)| graph.add_edge(a, b));
  println!(
    "{}",
    graph
      .scc()
      .into_iter()
      .map(|g| g.len() * (g.len() - 1) / 2)
      .sum::<usize>()
  )
}
