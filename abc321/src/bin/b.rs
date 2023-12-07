#![allow(non_snake_case)]
use proconio::{fastout, input};

#[fastout]
fn main() {
  input! {n:usize,x:i32,a:[i32;n-1]}

  let (min, max) = (*a.iter().min().unwrap(), *a.iter().max().unwrap());
  let nec = x + min + max - a.iter().sum::<i32>();
  println!(
    "{}",
    if nec <= min {
      0
    } else if nec > max {
      -1
    } else {
      nec.max(min)
    }
  );
}
