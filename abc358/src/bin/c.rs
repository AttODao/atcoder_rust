#![allow(non_snake_case)]
use itertools::Itertools;
use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
  input! {n:usize,m:usize,s:[Chars;n]}
  for i in 1..n {
    for shops in (0..n).combinations(i) {
      let mut ices = 0;
      for shop in shops {
        for ice in 0..m {
          if s[shop][ice] == 'o' {
            ices |= 1 << ice;
          }
        }
        if ices == (1 << m) - 1 {
          println!("{i}");
          return;
        }
      }
    }
  }
  println!("{n}");
}
