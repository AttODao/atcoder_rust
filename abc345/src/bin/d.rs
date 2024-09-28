#![allow(non_snake_case)]
use itertools::Itertools;
use proconio::{fastout, input};

fn mktile(a: i128, b: i128, w: i128) -> i128 {
  ((1 << a * (w + 1))- 1) / ((1 << w + 1) - 1) * ((1 << b) - 1)
}

fn tilepair((a, b): (i128, i128), w: i128) -> Vec<i128> {
  vec![mktile(a, b, w), mktile(b, a, w)]
}

#[fastout]
fn main() {
  input! {n:usize,w:i128,h:i128,ab:[(i128,i128);n]}
  for perm in ab.into_iter().map(|ab| tilepair(ab, w)).permutations(n) {
    for r in 0..1 << n {
      let mut grid = mktile(h, w, w);
      for tile in perm
        .iter()
        .enumerate()
        .map(|(i, tilepair)| tilepair[(r >> i) & 1])
      {
        if tile & grid != tile {
          break;
        }
        grid ^= tile;
        if grid == 0 {
          println!("Yes");
          return;
        }
        grid /= grid & -grid;
      }
    }
  }
  println!("No");
}
