#![allow(non_snake_case)]
use ac_library::Dsu;
use im_rc::HashSet;
use itertools::iproduct;
use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
  input! {h:usize,w:usize,mut s:[Chars;h]}
  for (i, j) in iproduct!(0..h, 0..w) {
    if s[i][j] == '#' {
      for (di, dj) in [(1, 0), (0, 1), (!0, 0), (0, !0)] {
        let (ni, nj) = (i.overflowing_add(di).0, j.overflowing_add(dj).0);
        if ni < h && nj < w && s[ni][nj] == '.' {
          s[ni][nj] = '!';
        }
      }
    }
  }
  let mut uf = Dsu::new(h * w);
  for (i, j) in iproduct!(0..h, 0..w) {
    if s[i][j] == '.' {
      for (di, dj) in [(1, 0), (0, 1)] {
        let (ni, nj) = (i + di, j + dj);
        if ni < h && nj < w && s[ni][nj] == '.' {
          uf.merge(i + j * h, ni + nj * h);
        }
      }
    }
  }
  let mut free = vec![1; w * h];
  for g in uf.groups() {
    free[uf.leader(g[0])] = uf.size(g[0]);
  }
  for (i, j) in iproduct!(0..h, 0..w) {
    if s[i][j] == '!' {
      let mut used = HashSet::new();
      for (di, dj) in [(1, 0), (0, 1), (!0, 0), (0, !0)] {
        let (ni, nj) = (i.overflowing_add(di).0, j.overflowing_add(dj).0);
        if ni < h && nj < w && s[ni][nj] == '.' && !used.contains(&uf.leader(ni + nj * h)) {
          free[uf.leader(ni + nj * h)] += 1;
          used.insert(uf.leader(ni + nj * h));
        }
      }
    }
  }
  println!("{}", free.into_iter().max().unwrap());
}
