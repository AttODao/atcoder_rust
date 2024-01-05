#![allow(non_snake_case)]
use proconio::{fastout, input};

#[fastout]
fn main() {
  input! {p:char,q:char}
  let dist = [0u32, 3, 4, 8, 9, 14, 23];
  println!(
    "{}",
    dist[p as usize - 'A' as usize].abs_diff(dist[q as usize - 'A' as usize])
  );
}
