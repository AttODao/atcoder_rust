#![allow(non_snake_case)]
use itertools::Itertools;
use proconio::{fastout, input, marker::Usize1};

#[fastout]
fn main() {
  input! {n:usize,m:usize,abc:[(Usize1,Usize1,i64);m]}

  let mut edges = vec![vec![-1; n]; n];
  for (a, b, c) in abc {
    edges[a][b] = c;
    edges[b][a] = c;
  }
  let mut ans = 0;
  for p in (0..n).permutations(n) {
    let mut dist = 0;
    for (a, b) in p.into_iter().tuple_windows() {
      if edges[a][b] > 0 {
        dist += edges[a][b];
        ans = ans.max(dist);
      } else {
        break;
      }
    }
  }
  println!("{}", ans);
}
