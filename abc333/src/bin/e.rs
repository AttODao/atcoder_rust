#![allow(non_snake_case)]
use itertools::Itertools;
use proconio::{fastout, input, marker::Usize1};

#[fastout]
fn main() {
  input! {n:usize,tx:[(Usize1,Usize1);n]}

  let mut p = vec![vec![]; n];
  for &(t, x) in &tx {
    if t == 0 {
      p[x].push(1);
    } else {
      p[x].push(0);
    }
  }
  for p in p.iter_mut() {
    p.push(0);
    for i in (0..p.len() - 1).rev() {
      if p[i] == 0 {
        p[i] = p[i + 1] + 1;
      } else {
        p[i] = 0.max(p[i + 1] - 1);
      }
    }
    if p[0] != 0 {
      println!("-1");
      return;
    }
  }
  let (mut k, mut kmax) = (0, 0);
  let mut ans = vec![];
  let mut pi = vec![0; n];
  for &(t, x) in &tx {
    if t == 0 {
      ans.push(p[x][pi[x] + 1] - p[x][pi[x]]);
      if ans.last().unwrap() == &1 {
        k += 1;
        kmax = kmax.max(k);
      }
    } else {
      k -= 1;
    }
    pi[x] += 1;
  }
  println!("{}\n{}", kmax, ans.iter().join(" "));
}
