use ac_library::SccGraph;
use itertools::Itertools;
use proconio::{fastout, input};

#[fastout]
fn main() {
  input! {n:usize,m:usize,ab:[(usize,usize);m]}

  let mut graph = SccGraph::new(n);
  ab.into_iter().for_each(|(a, b)| graph.add_edge(a, b));
  let res = graph.scc();
  println!(
    "{}\n{}",
    res.len(),
    res
      .into_iter()
      .map(|res| format!("{} {}", res.len(), res.iter().rev().join(" ")))
      .join("\n")
  );
}
