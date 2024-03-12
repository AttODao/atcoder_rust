#![allow(non_snake_case)]

use std::collections::BinaryHeap;

use itertools::Itertools;
use proconio::{fastout, input, marker::Usize1};

fn dep(arr: u64, l: u64, d: u64, k: u64, c: u64) -> u64 {
  if arr < l + c {
    0
  } else if arr < l + k * d + c {
    let i = (arr - l - c) / d;
    l + i * d
  } else {
    l + (k - 1) * d
  }
}

#[fastout]
fn main() {
  input! {n:usize,m:usize,ldkcab:[(u64,u64,u64,u64,Usize1,Usize1);m]}
  let mut edges = vec![vec![]; n];
  for (l, d, k, c, a, b) in ldkcab {
    edges[b].push((a, l, d, k, c));
  }
  let mut f = vec![0; n];
  let mut heap = BinaryHeap::new();
  f[n - 1] = u64::MAX;
  heap.push((u64::MAX, n - 1));
  while let Some((fv, v)) = heap.pop() {
    if f[v] > fv {
      continue;
    }
    for &(u, l, d, k, c) in &edges[v] {
      let tmpfu = dep(fv, l, d, k, c);
      if f[u] < tmpfu {
        f[u] = tmpfu;
        heap.push((tmpfu, u));
      }
    }
  }
  println!(
    "{}",
    f[..n - 1]
      .iter()
      .map(|&f| if f == 0 {
        "Unreachable".to_string()
      } else {
        f.to_string()
      })
      .join("\n")
  );
}
