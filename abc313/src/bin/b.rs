#![allow(non_snake_case)]
use itertools::Itertools;
use proconio::{fastout, input, marker::Usize1};

#[fastout]
fn main() {
  input! {n:usize,ab:[(Usize1,Usize1)]}

  let mut saikyo = vec![true; n];
  for (_, b) in ab {
    saikyo[b] = false;
  }
  let saikyo = (0..n).filter(|&i| saikyo[i]).collect_vec();
  if saikyo.len() == 1 {
    println!("{}", saikyo[0] + 1);
  } else {
    println!("-1");
  }
}
