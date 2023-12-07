use ac_library::MinCostFlowGraph;
use itertools::{iproduct, Itertools};
use proconio::{fastout, input};

const BIG: i64 = 1000000000;

#[fastout]
fn main() {
  input! {n:usize,k:i64,a:[[i64;n];n]}

  let mut mcf = MinCostFlowGraph::<i64>::new(2 * n + 2);
  mcf.add_edge(2 * n, 2 * n + 1, BIG, BIG);
  let mut edges = vec![];
  for i in 0..n {
    mcf.add_edge(2 * n, i, k, 0);
    mcf.add_edge(n + i, 2 * n + 1, k, 0);
  }
  for (i, j) in iproduct!(0..n, 0..n) {
    edges.push(mcf.add_edge(i, n + j, 1, BIG - a[i][j]));
  }
  let res = mcf.flow(2 * n, 2 * n + 1, BIG);
  println!("{}", BIG * res.0 - res.1);
  let mut state = vec![vec!['.'; n]; n];
  for e in edges {
    let e = mcf.get_edge(e);
    if e.flow > 0 {
      state[e.from][e.to - n] = 'X';
    }
  }
  println!(
    "{}",
    state
      .into_iter()
      .map(|s| s.iter().collect::<String>())
      .join("\n")
  );
}
