#![allow(non_snake_case)]
use proconio::{fastout, input};

#[fastout]
fn main() {
  input! {m:usize,d:[usize;m]}

  let mut mid = (d.iter().sum::<usize>() + 1) / 2;
  for (m, d) in d.into_iter().enumerate() {
    if d < mid {
      mid -= d;
    } else {
      println!("{} {}", m + 1, mid);
      break;
    }
  }
}
