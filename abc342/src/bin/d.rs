#![allow(non_snake_case)]
use im_rc::HashMap;
use itertools::Itertools;
use proconio::{fastout, input};

#[fastout]
fn main() {
  input! {n:usize,a:[u32;n]}
  let mut dpa = HashMap::new();
  let mut zero = 0;
  for a in a {
    if a == 0 {
      zero += 1;
    } else {
      for d in (1..500).rev().map(|i| i * i) {
        if a % (d) == 0 {
          *dpa.entry(a / d).or_insert(0) += 1;
          break;
        }
      }
    }
  }
  println!(
    "{}",
    dpa.values().map(|v| v * (v - 1) / 2).sum::<usize>() + n * (n - 1) / 2
      - (n - zero) * (n - zero - 1) / 2
  );
}
