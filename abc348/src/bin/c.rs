#![allow(non_snake_case)]
use im_rc::HashMap;
use proconio::{fastout, input};

#[fastout]
fn main() {
  input! {ac:[(u64,u64)]}
  let mut amin = HashMap::new();
  for (a, c) in ac {
    let b = amin.entry(c).or_insert(u64::MAX);
    if a < *b {
      *b = a;
    }
  }
  println!("{}", amin.values().max().unwrap())
}
