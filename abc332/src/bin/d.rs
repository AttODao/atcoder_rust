#![allow(non_snake_case)]

use itertools::{iproduct, Itertools};
use proconio::{fastout, input};

#[fastout]
fn main() {
  input! {h:usize,w:usize,a:[[u32;w];h],b:[[u32;w];h]}

  let pi = (0..h)
    .permutations(h)
    .map(|p| {
      let mut np = p.clone();
      let mut a = 0;
      for i in (0..h).rev() {
        for j in 0..i {
          if np[j] > np[j + 1] {
            np.swap(j, j + 1);
            a += 1;
          }
        }
      }
      (p, a)
    })
    .collect_vec();
  let pj = (0..w).permutations(w).map(|p| {
    let mut np = p.clone();
    let mut a = 0;
    for i in (0..w).rev() {
      for j in 0..i {
        if np[j] > np[j + 1] {
          np.swap(j, j + 1);
          a += 1;
        }
      }
    }
    (p, a)
  });
  let mut ans = u32::MAX;
  for ((pi, ai), (pj, aj)) in iproduct!(pi, pj) {
    if iproduct!(0..h, 0..w).all(|(i, j)| a[pi[i]][pj[j]] == b[i][j]) {
      ans = ans.min(ai + aj);
    }
  }
  println!(
    "{}",
    if ans < u32::MAX {
      ans.to_string()
    } else {
      "-1".to_string()
    }
  );
}
