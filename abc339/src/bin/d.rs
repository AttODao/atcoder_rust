#![allow(non_snake_case)]
use std::collections::VecDeque;

use itertools::{iproduct, Itertools};
use proconio::{fastout, input, marker::Chars};

const D: [(i32, i32); 4] = [(1, 0), (-1, 0), (0, 1), (0, -1)];

#[fastout]
fn main() {
  input! {n:usize,s:[Chars;n]}
  let ((ai, aj), (bi, bj)) = iproduct!(0..n as i32, 0..n as i32)
    .filter(|&(i, j)| s[i as usize][j as usize] == 'P')
    .collect_tuple()
    .unwrap();
  let mut discovered = vec![vec![vec![vec![false; n]; n]; n]; n];
  let mut bfs = VecDeque::new();
  discovered[ai as usize][aj as usize][bi as usize][bj as usize] = true;
  bfs.push_back(((ai, aj), (bi, bj), 0));
  while let Some(((ai, aj), (bi, bj), cnt)) = bfs.pop_front() {
    if (ai, aj) == (bi, bj) {
      println!("{}", cnt);
      return;
    }
    for &(di, dj) in D.iter() {
      let mut nai = ai + di;
      let mut naj = aj + dj;
      if nai < 0
        || n as i32 <= nai
        || naj < 0
        || n as i32 <= naj
        || s[nai as usize][naj as usize] == '#'
      {
        nai -= di;
        naj -= dj;
      }
      let mut nbi = bi + di;
      let mut nbj = bj + dj;
      if nbi < 0
        || n as i32 <= nbi
        || nbj < 0
        || n as i32 <= nbj
        || s[nbi as usize][nbj as usize] == '#'
      {
        nbi -= di;
        nbj -= dj;
      }
      if !discovered[nai as usize][naj as usize][nbi as usize][nbj as usize] {
        discovered[nai as usize][naj as usize][nbi as usize][nbj as usize] = true;
        bfs.push_back(((nai, naj), (nbi, nbj), cnt + 1));
      }
    }
  }
  println!("-1");
}
