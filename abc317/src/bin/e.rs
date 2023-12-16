#![allow(non_snake_case)]
use std::{collections::VecDeque, iter};

use itertools::{iproduct, izip, Itertools};
use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
  input! {h:usize,w:usize,mut a:[Chars;h]}

  for (i, j) in iproduct!(0..h, 0..w) {
    for (ni, nj) in match a[i][j] {
      'v' => izip!(i + 1..h, iter::repeat(j)).collect_vec(),
      '^' => izip!((0..i).rev(), iter::repeat(j)).collect_vec(),
      '>' => izip!(iter::repeat(i), j + 1..w).collect_vec(),
      '<' => izip!(iter::repeat(i), (0..j).rev()).collect_vec(),
      _ => vec![],
    } {
      if ['#', 'v', '^', '>', '<'].contains(&a[ni][nj]) {
        break;
      }
      a[ni][nj] = '!';
    }
  }
  let mut dist = vec![vec![usize::MAX; w]; h];
  let mut bfs = VecDeque::new();
  let (si, sj) = iproduct!(0..h, 0..w)
    .find(|&(i, j)| a[i][j] == 'S')
    .unwrap();
  bfs.push_back((si, sj));
  dist[si][sj] = 0;
  while let Some((i, j)) = bfs.pop_front() {
    let mut next = vec![];
    if i > 0 {
      next.push((i - 1, j));
    }
    if i < h - 1 {
      next.push((i + 1, j));
    }
    if j > 0 {
      next.push((i, j - 1));
    }
    if j < w - 1 {
      next.push((i, j + 1));
    }
    let ndist = dist[i][j] + 1;
    for (ni, nj) in next {
      if a[ni][nj] == 'G' {
        println!("{}", ndist);
        return;
      } else if a[ni][nj] == '.' && dist[ni][nj] > ndist {
        dist[ni][nj] = ndist;
        bfs.push_back((ni, nj));
      }
    }
  }
  println!("-1");
}
