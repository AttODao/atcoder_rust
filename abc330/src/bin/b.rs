use std::cmp;

use proconio::{fastout, input};

#[fastout]
fn main() {
  input! {n:usize,l:i32,r:i32,a:[i32;n]}

  for a in a {
    match (a.cmp(&l), a.cmp(&r)) {
      (cmp::Ordering::Less, cmp::Ordering::Less) => {
        print!("{} ", l);
      }
      (cmp::Ordering::Greater, cmp::Ordering::Greater) => {
        print!("{} ", r);
      }
      _ => {
        print!("{} ", a);
      }
    }
  }
}
