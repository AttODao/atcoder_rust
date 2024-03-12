#![allow(non_snake_case)]
use std::mem;

use itertools::Itertools;
use proconio::{fastout, input};

#[fastout]
fn main() {
  input! {h:i32,w:i32,n:i32}
  let mut grid = vec![vec![false; w as usize]; h as usize];
  let (mut x, mut y) = (0, 0);
  let (mut dx, mut dy) = (-1, 0);
  for _ in 0..n {
    mem::swap(&mut dx, &mut dy);
    if grid[x as usize][y as usize] {
      dx = -dx;
    } else {
      dy = -dy;
    }
    grid[x as usize][y as usize] = !grid[x as usize][y as usize];
    x = (x + dx + h) % h;
    y = (y + dy + w) % w;
  }
  println!(
    "{}",
    grid
      .into_iter()
      .map(|g| g.into_iter().map(|g| if g { '#' } else { '.' }).join(""))
      .join("\n")
  );
}
