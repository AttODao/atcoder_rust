#![allow(non_snake_case)]

use proconio::{fastout, input};

#[fastout]
fn main() {
  input! {n:usize}

  let (mut a, mut b, mut c) = (0, 0, 0);
  for _ in 1..n {
    if c < b {
      c += 1;
    } else if b < a {
      b += 1;
      c = 0;
    } else {
      a += 1;
      b = 0;
      c = 0;
    }
  }
  println!(
    "{}",
    "1".repeat(a - b) + &"2".repeat(b - c) + &"3".repeat(c + 1)
  )
}
