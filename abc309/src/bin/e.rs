#![allow(non_snake_case)]
use proconio::{fastout, input, marker::Usize1};

#[fastout]
fn main() {
  input! {n:usize,m:usize,p:[Usize1;n-1],xy:[(Usize1,i32);m]}
  let mut edges = vec![vec![]; n];
  for (i, p) in p.into_iter().enumerate() {
    edges[p].push(i + 1);
  }
  let mut ins = vec![-1; n];
  for (x, y) in xy {
    ins[x] = ins[x].max(y);
  }
  let mut ans = 0;
  let mut dfs = vec![];
  dfs.push((0, 0));
  while let Some((now, rem)) = dfs.pop() {
    let ins = ins[now].max(rem - 1);
    if ins >= 0 {
      ans += 1;
    }
    for &next in &edges[now] {
      dfs.push((next, ins));
    }
  }
  println!("{}", ans);
}
