#![allow(non_snake_case)]
use ac_library::Dsu;
use itertools::Itertools;
use proconio::{fastout, input};

#[fastout]
fn main() {
  input! {n:usize,d:i32,xy:[(i32,i32);n]}
  let mut dsu = Dsu::new(n);
  for ((i, (x1, y1)), (j, (x2, y2))) in xy.into_iter().enumerate().tuple_combinations() {
    if (x1 - x2).pow(2) + (y1 - y2).pow(2) <= d * d {
      dsu.merge(i, j);
    }
  }
  println!(
    "{}",
    (0..n)
      .map(|i| if dsu.same(0, i) { "Yes" } else { "No" })
      .join("\n")
  );
}
