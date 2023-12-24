#![allow(non_snake_case)]
use ac_library::{Dsu, ModInt998244353};
use im_rc::HashSet;
use itertools::iproduct;
use proconio::{fastout, input, marker::Chars};

type Mint = ModInt998244353;

fn index(i: usize, j: usize, w: usize) -> usize {
  i * w + j
}

#[fastout]
fn main() {
  input! {h:usize,w:usize,s:[Chars;h]}

  let mut uf = Dsu::new(h * w);
  for (i, j) in iproduct!(0..h, 0..w) {
    if s[i][j] == '#' {
      if i < h - 1 && s[i + 1][j] == '#' {
        uf.merge(index(i, j, w), index(i + 1, j, w));
      }
      if j < w - 1 && s[i][j + 1] == '#' {
        uf.merge(index(i, j, w), index(i, j + 1, w));
      }
    }
  }
  let mut nred: Mint = 0.into();
  let mut sum: Mint = 0.into();
  let ngroup = uf.groups().iter().filter(|&gr| {
    let index = gr[0];
    let (i, j) = (index / w, index % w);
    s[i][j]=='#'
  }).count();
  for (i, j) in iproduct!(0..h, 0..w) {
    if s[i][j] == '.' {
      nred += 1;
      let mut groupset = HashSet::new();
      if i > 0 && s[i - 1][j] == '#' {
        groupset.insert(uf.leader(index(i - 1, j, w)));
      }
      if i < h - 1 && s[i + 1][j] == '#' {
        groupset.insert(uf.leader(index(i + 1, j, w)));
      }
      if j > 0 && s[i][j - 1] == '#' {
        groupset.insert(uf.leader(index(i, j - 1, w)));
      }
      if j < w - 1 && s[i][j + 1] == '#' {
        groupset.insert(uf.leader(index(i, j + 1, w)));
      }
      sum += ngroup + 1 - groupset.len();
    }
  }
  println!("{}", sum / nred);
}
