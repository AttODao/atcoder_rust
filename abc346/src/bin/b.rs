#![allow(non_snake_case)]
use itertools::Itertools;
use proconio::{fastout, input};

#[fastout]
fn main() {
  let key = "wbwbwwbwbwbw";
  input! {w:usize,b:usize}
  for k in key.repeat(20).chars().collect_vec().windows(w + b).take(12) {
    if k.iter().map(|&c| (c == 'w') as usize).sum::<usize>() == w {
      println!("Yes");
      return;
    }
  }
  println!("No");
}
