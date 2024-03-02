#![allow(non_snake_case)]
use itertools::Itertools;
use proconio::{fastout, input, marker::Usize1};

#[fastout]
fn main() {
  input! {n:usize,m:usize,a:[[Usize1;n];m]}
  let mut discord = vec![vec![true; n]; n];
  for a in a {
    for (x, y) in a.into_iter().tuple_windows() {
      discord[x][y] = false;
      discord[y][x] = false;
    }
  }
  println!(
    "{}",
    (discord.into_iter().flatten().filter(|&d| d).count() - n) / 2
  );
}
