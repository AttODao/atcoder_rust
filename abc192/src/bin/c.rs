use itertools::Itertools;
#[allow(non_camel_case_types, non_snake_case)]
use proconio::{fastout, input};

fn g1(x: usize) -> usize {
  return x
    .to_string()
    .chars()
    .sorted()
    .rev()
    .collect::<String>()
    .parse::<usize>()
    .unwrap();
}
fn g2(x: usize) -> usize {
  return x
    .to_string()
    .chars()
    .sorted()
    .collect::<String>()
    .parse::<usize>()
    .unwrap();
}

#[fastout]
fn main() {
  input! {mut n:usize,k:usize}

  for _ in 0..k {
    n = g1(n) - g2(n);
  }
  println!("{}", n);
}
