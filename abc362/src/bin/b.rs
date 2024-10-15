#![allow(non_snake_case)]
use itertools::Itertools;
use proconio::{fastout, input};

#[fastout]
fn main() {
  input! {p:[(i32,i32);3]}
  let l = p
    .into_iter()
    .circular_tuple_windows()
    .map(|((x1, y1), (x2, y2))| (x1 - x2) * (x1 - x2) + (y1 - y2) * (y1 - y2))
    .sorted()
    .collect_vec();
  if l[0] + l[1] == l[2] {
    println!("Yes");
  } else {
    println!("No");
  }
}
