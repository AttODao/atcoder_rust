#![allow(non_snake_case)]
use std::collections::VecDeque;

use proconio::{fastout, input, marker::Usize1};

#[fastout]
fn main() {
  input! {q:usize}
  let mut a = VecDeque::new();
  for _ in 0..q {
    input! {t:u8}
    if t == 1 {
      input! {x:u32}
      a.push_front(x);
    } else {
      input! {k:Usize1}
      println!("{}", a[k]);
    }
  }
}
