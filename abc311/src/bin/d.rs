#![allow(non_snake_case)]
use std::collections::VecDeque;

use im_rc::hashset;
use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
  input! {n:usize,m:usize,s:[Chars;n]}
  let mut bfs = VecDeque::new();
  let mut discovered = hashset![];
  let mut path = vec![vec![false; m]; n];
  bfs.push_back((1, 1));
  discovered.insert((1, 1));
  while let Some((i, j)) = bfs.pop_front() {
    path[i][j] = true;
    for &(di, dj) in [(1, 0), (0, 1)].iter() {
      {
        let (mut i, mut j) = (i, j);
        while s[i + di][j + dj] == '.' {
          path[i][j] = true;
          i += di;
          j += dj;
        }
        if !discovered.contains(&(i, j)) {
          bfs.push_back((i, j));
          discovered.insert((i, j));
        }
      }
      {
        let (mut i, mut j) = (i, j);
        while s[i - di][j - dj] == '.' {
          path[i][j] = true;
          i -= di;
          j -= dj;
        }
        if !discovered.contains(&(i, j)) {
          bfs.push_back((i, j));
          discovered.insert((i, j));
        }
      }
    }
  }
  println!("{}", path.iter().flatten().filter(|&&p| p).count());
}
