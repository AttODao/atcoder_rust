#![allow(non_snake_case)]
use proconio::{
  fastout, input,
  marker::{Chars, Usize1},
};

#[fastout]
fn main() {
  input! {h:usize,w:usize,mut i:Usize1,mut j:Usize1,c:[Chars;h],x:Chars}
  for x in x {
    (i, j) = match x {
      'L' => {
        if j > 0 && c[i][j - 1] == '.' {
          (i, j - 1)
        } else {
          (i, j)
        }
      }
      'R' => {
        if j < w - 1 && c[i][j + 1] == '.' {
          (i, j + 1)
        } else {
          (i, j)
        }
      }
      'U' => {
        if i > 0 && c[i - 1][j] == '.' {
          (i - 1, j)
        } else {
          (i, j)
        }
      }
      'D' => {
        if i < h - 1 && c[i + 1][j] == '.' {
          (i + 1, j)
        } else {
          (i, j)
        }
      }
      _ => unreachable!(),
    };
  }
  println!("{} {}", i + 1, j + 1);
}
