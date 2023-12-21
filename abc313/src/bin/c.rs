#![allow(non_snake_case)]
use proconio::{fastout, input};

#[fastout]
fn main() {
  input! {n:usize,a:[usize;n]};

  let sum = a.iter().sum::<usize>();
  let ave1 = sum / n;
  let ave2 = (sum + n - 1) / n;
  println!(
    "{}",
    a.iter()
      .map(|&a| ave1.saturating_sub(a))
      .sum::<usize>()
      .max(a.iter().map(|&a| a.saturating_sub(ave2)).sum::<usize>())
  )
}
