#![allow(non_snake_case)]
use proconio::{fastout, input};

#[fastout]
fn main() {
  input! {k:usize,g:i64,m:i64}

  let (mut glass, mut mag) = (0, 0);
  for _ in 0..k {
    if glass == g {
      glass = 0;
    } else if mag == 0 {
      mag = m;
    } else {
      let ng = glass;
      glass += mag;
      glass = glass.min(g);
      mag -= g - ng;
      mag = mag.max(0);
    }
  }
  println!("{} {}", glass, mag);
}
