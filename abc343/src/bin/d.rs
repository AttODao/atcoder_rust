#![allow(non_snake_case)]
use std::collections::HashMap;

use proconio::{fastout, input, marker::Usize1};

#[fastout]
fn main() {
  input! {n:usize,t:usize,ab:[(Usize1,u64);t]}
  let mut score = vec![0; n];
  let mut cnt = HashMap::new();
  cnt.insert(0, n);
  let mut div = 1;
  for (a, b) in ab {
    *cnt.entry(score[a]).or_insert(0) -= 1;
    if cnt[&score[a]] == 0 {
      div -= 1;
    }
    score[a] += b;
    *cnt.entry(score[a]).or_insert(0) += 1;
    if cnt[&score[a]] == 1 {
      div += 1;
    }
    println!("{}", div);
  }
}
