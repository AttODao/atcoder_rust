#![allow(non_snake_case)]
use itertools::Itertools;
use proconio::{fastout, input};

#[fastout]
fn main() {
  input! {n:usize,a:u32,b:u32,d:[u32;n]}
  let mut d = d
    .into_iter()
    .map(|d| d % (a + b))
    .sorted()
    .dedup()
    .collect_vec();
  d.push(d[0] + a + b);
  println!(
    "{}",
    if d.into_iter().tuple_windows().any(|(d1, d2)| d2 - d1 > b) {
      "Yes"
    } else {
      "No"
    }
  );
}
