#![allow(non_snake_case)]
use itertools::Itertools;
use proconio::{fastout, input};

#[fastout]
fn main() {
  input! {n:usize,q:usize}
  let mut path = (1..=n).rev().map(|i| (i as i32, 0)).collect_vec();
  for _ in 0..q {
    input! {t:u8}
    if t == 1 {
      input! {c:char}
      let mut next = path.last().unwrap().clone();
      match c {
        'R' => next.0 += 1,
        'L' => next.0 -= 1,
        'U' => next.1 += 1,
        'D' => next.1 -= 1,
        _ => unreachable!(),
      }
      path.push(next);
    } else {
      input! {p:usize}
      let (x, y) = path[path.len() - p];
      println!("{} {}", x, y);
    }
  }
}
