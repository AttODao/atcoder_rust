use std::{cmp::Reverse, collections::BinaryHeap};

use proconio::{fastout, input, marker::Usize1};

#[fastout]
fn main() {
  input! {n:usize,m:usize,x:Usize1,y:Usize1,abtk:[(Usize1,Usize1,isize,isize);m]}

  let mut graph = vec![vec![]; n];
  for (a, b, t, k) in abtk {
    graph[a].push((b, t, k));
    graph[b].push((a, t, k));
  }

  let mut dist = vec![-1; n];
  let mut table = BinaryHeap::new();
  table.push((Reverse(0), x, 1 << 30));
  while let Some((Reverse(time), now, prev)) = table.pop() {
    if dist[now] != -1 {
      continue;
    }
    dist[now] = time;
    for &(next, t, k) in graph[now].iter() {
      if next != prev {
        table.push((Reverse((time + k - 1) / k * k + t), next, now));
      }
    }
  }
  println!("{}", dist[y]);
}
