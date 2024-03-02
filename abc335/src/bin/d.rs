#![allow(non_snake_case)]
use itertools::Itertools;
use proconio::{fastout, input};

#[fastout]
fn main() {
  input! {n:usize}
  let mut grid = vec![vec!["A".to_string(); n + 2]; n + 2];
  grid[(n + 1) / 2][(n + 1) / 2] = "T".to_string();
  for i in 0..n + 2 {
    grid[i][0] = "W".to_string();
    grid[i][n + 1] = "W".to_string();
    grid[0][i] = "W".to_string();
    grid[n + 1][i] = "W".to_string();
  }
  let (mut i, mut j) = (1i32, 1i32);
  let mut index: u32 = 1;
  for (di, dj) in vec![(1, 0), (0, 1), (-1, 0), (0, -1)].into_iter().cycle() {
    while grid[i as usize][j as usize] == "A" {
      grid[i as usize][j as usize] = index.to_string();
      index += 1;
      i = i + di;
      j = j + dj;
    }
    if grid[i as usize][j as usize] == "T" {
      break;
    }
    i -= di;
    j -= dj;
    index -= 1;
    grid[i as usize][j as usize] = "A".to_string();
  }
  println!(
    "{}",
    grid[1..n + 1]
      .into_iter()
      .map(|g| g[1..n + 1].into_iter().join(" "))
      .join("\n")
  );
}
