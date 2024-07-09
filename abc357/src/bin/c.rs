#![allow(non_snake_case)]
use itertools::Itertools;
use proconio::{fastout, input};

#[fastout]
fn main() {
  input! {n:u32}
  println!(
    "{}",
    (0..3i32.pow(n))
      .map(|i| (0..3i32.pow(n))
        .map(|j| {
          if (0..n)
            .map(|k| 3i32.pow(k))
            .any(|k| ((i / k) % 3 == 1) && ((j / k) % 3 == 1))
          {
            '.'
          } else {
            '#'
          }
        })
        .join(""))
      .join("\n")
  );
}
