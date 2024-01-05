#![allow(non_snake_case)]
use proconio::{fastout, input, marker::Usize1};

#[fastout]
fn main() {
  input! {n:usize,a:[Usize1;3*n]}
  let mut cnt = vec![0; n];
  for a in a {
    cnt[a] += 1;
    if cnt[a] == 2 {
      print!("{} ", a + 1);
    }
  }
}
