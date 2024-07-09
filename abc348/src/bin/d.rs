#![allow(non_snake_case)]
use std::collections::BinaryHeap;

use im_rc::HashMap;
use itertools::iproduct;
use proconio::{
  fastout, input,
  marker::{Chars, Usize1},
};

#[fastout]
fn main() {
  input! {h:usize,w:usize,a:[Chars;h],rce:[(Usize1,Usize1,isize)]}
  let medicines = rce
    .into_iter()
    .map(|(r, c, e)| ((r, c), e))
    .collect::<HashMap<_, _>>();
  let mut bfs = BinaryHeap::new();
  bfs.push((
    0,
    iproduct!(0..h, 0..w)
      .find(|&(i, j)| a[i][j] == 'S')
      .unwrap(),
  ));
  let mut energy = vec![vec![-1; w]; h];
  while let Some((mut e, (i, j))) = bfs.pop() {
    if e > energy[i][j] {
      if a[i][j] == 'T' {
        println!("Yes");
        return;
      }
      e = e.max(*medicines.get(&(i, j)).unwrap_or(&0));
      energy[i][j] = e;
      for (di, dj) in [(1, 0), (0, 1), (!0, 0), (0, !0)] {
        let (ni, nj) = (i.overflowing_add(di).0, j.overflowing_add(dj).0);
        if ni < h && nj < w && a[ni][nj] != '#' {
          bfs.push((e - 1, (ni, nj)));
        }
      }
    }
  }
  println!("No");
}
