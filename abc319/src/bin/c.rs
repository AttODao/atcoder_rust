#![allow(non_snake_case)]
use itertools::{iproduct, Itertools};
use proconio::{fastout, input};

#[fastout]
fn main() {
  input! {c:[i8;9]}

  let straight = vec![
    (0, 1, 2),
    (3, 4, 5),
    (6, 7, 8),
    (0, 3, 6),
    (1, 4, 7),
    (2, 5, 8),
    (0, 4, 8),
    (2, 4, 6),
  ];
  let mut cnt = 0;
  for p in (0..9).permutations(9) {
    let mut flag = true;
    for &(x, y, z) in &straight {
      if c[y] == c[z] && p[y] < p[x] && p[z] < p[x]
        || c[z] == c[x] && p[z] < p[y] && p[x] < p[y]
        || c[x] == c[y] && p[x] < p[z] && p[y] < p[z]
      {
        flag = false;
      }
    }
    if flag {
      cnt += 1;
    }
  }
  println!("{:.16}", cnt as f64 / (1..=9).product::<u64>() as f64);
}
