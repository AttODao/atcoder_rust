#![allow(non_snake_case)]
use std::collections::HashMap;

use itertools::Itertools;
use proconio::{fastout, input};

#[fastout]
fn main() {
  input! {mut a:[u32],q:usize};
  let mut front = HashMap::new();
  let mut back = HashMap::new();
  a.insert(0, 0);
  a.push(0);
  for (&x, &y) in a.iter().tuple_windows() {
    front.insert(x, y);
    back.insert(y, x);
  }
  for _ in 0..q {
    input! {t:u8}
    if t == 1 {
      input! {x:u32,y:u32}
      let z = front[&x];
      front.insert(x, y);
      front.insert(y, z);
      back.insert(y, x);
      back.insert(z, y);
    } else {
      input! {y:u32}
      let (x, z) = (back[&y], front[&y]);
      front.insert(x, z);
      back.insert(z, x);
    }
  }
  let mut i = front[&0];
  while i != 0 {
    print!("{} ", i);
    i = front[&i];
  }
}
