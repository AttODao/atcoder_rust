#![allow(non_snake_case)]

use itertools::{iproduct, Itertools};
use maplit::hashset;
use proconio::{fastout, input};

#[fastout]
fn main() {
  input! {c:[(usize,usize,usize,usize,usize,usize)]}

  let mut grid = vec![vec![vec![None; 101]; 101]; 101];
  for (i, &(x1, y1, z1, x2, y2, z2)) in c.iter().enumerate() {
    for (x, y, z) in iproduct!(x1..x2, y1..y2, z1..z2) {
      grid[x][y][z] = Some(i);
    }
  }
  let mut sets = (0..c.len()).map(|i| hashset! {i}).collect_vec();
  for (x, y, z) in iproduct!(0..100, 0..100, 0..100) {
    if let Some(a) = grid[x][y][z] {
      for &(dx, dy, dz) in [(1, 0, 0), (0, 1, 0), (0, 0, 1)].iter() {
        if let Some(b) = grid[x + dx][y + dy][z + dz] {
          sets[a].insert(b);
          sets[b].insert(a);
        }
      }
    }
  }
  println!("{}", sets.iter().map(|set| set.len() - 1).join("\n"));
}
