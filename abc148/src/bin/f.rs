use proconio::{fastout, input, marker::Usize1};
use std::cmp::max;

fn dfs(now: usize, prev: usize, dist: &mut Vec<usize>, graph: &Vec<Vec<usize>>) {
  let nowdist = dist[now];
  for next in &graph[now] {
    if *next != prev {
      dist[*next] = nowdist + 1;
      dfs(*next, now, dist, graph);
    }
  }
}

#[fastout]
fn main() {
  input! {n:usize,u:Usize1,v:Usize1,ab:[(Usize1,Usize1);n-1]}

  let mut graph: Vec<Vec<usize>> = vec![vec![]; n];
  for (a, b) in ab {
    graph[a].push(b);
    graph[b].push(a);
  }
  let (mut udist, mut vdist) = (vec![0; n], vec![0; n]);
  dfs(u, 100010, &mut udist, &graph);
  dfs(v, 100010, &mut vdist, &graph);
  let mut ans = 0;
  for i in 0..n {
    if udist[i] < vdist[i] {
      ans = max(ans, vdist[i]);
    }
  }
  println!("{}", ans - 1);
}
