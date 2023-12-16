#![allow(non_snake_case)]
use std::collections::BTreeSet;

use itertools::iproduct;
use proconio::{fastout, input, marker::Chars};

fn charindex(c: char) -> usize {
  c as usize - 'a' as usize
}

#[fastout]
fn main() {
  input! {h:usize,w:usize,c:[Chars;h]}

  let mut row = vec![vec![0; 26]; h];
  let mut col = vec![vec![0; 26]; w];
  for (i, j) in iproduct!(0..h, 0..w) {
    let index = charindex(c[i][j]);
    row[i][index] += 1;
    col[j][index] += 1;
  }
  let mut remrow = (0..h).collect::<BTreeSet<usize>>();
  let mut remcol = (0..w).collect::<BTreeSet<usize>>();
  loop {
    let mut checkrow = vec![];
    let mut checkcol = vec![];
    let nremrow = remrow.len();
    let nremcol = remcol.len();
    if nremcol > 1 {
      for &i in &remrow {
        for c in 0..26 {
          if row[i][c] == nremcol {
            checkrow.push((i, c));
          }
        }
      }
    }
    if nremrow > 1 {
      for &j in &remcol {
        for c in 0..26 {
          if col[j][c] == nremrow {
            checkcol.push((j, c));
          }
        }
      }
    }
    if checkrow.is_empty() && checkcol.is_empty() {
      break;
    }
    for (i, c) in checkrow {
      for &j in &remcol {
        col[j][c] -= 1;
      }
      remrow.remove(&i);
    }
    for (j, c) in checkcol {
      for &i in &remrow {
        row[i][c] -= 1;
      }
      remcol.remove(&j);
    }
  }
  println!("{}", remrow.len() * remcol.len());
}
