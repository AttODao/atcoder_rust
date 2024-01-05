#![allow(non_snake_case)]
use std::collections::VecDeque;

use itertools::Itertools;
use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
  input! {h:usize,w:usize,s:[Chars;h]}
  if s[0][0] != 's' {
    println!("No");
    return;
  }
  let mut bfs = VecDeque::<(usize, usize)>::new();
  let mut discovered = vec![vec![false; w]; h];
  bfs.push_back((0, 0));
  discovered[0][0] = true;
  while let Some((i, j)) = bfs.pop_front() {
    for (di, dj) in [(1, 0), (-1, 0), (0, 1), (0, -1)] {
      let (ni, nj) = (i.saturating_add_signed(di), j.saturating_add_signed(dj));
      if ni < h
        && nj < w
        && "snukes"
          .chars()
          .tuple_windows::<(char, char)>()
          .any(|a| a == (s[i][j], s[ni][nj]))
        && !discovered[ni][nj]
      {
        bfs.push_back((ni, nj));
        discovered[ni][nj] = true;
      }
    }
  }
  println!(
    "{}",
    if discovered[h - 1][w - 1] {
      "Yes"
    } else {
      "No"
    }
  );
}
