#![allow(non_snake_case)]
use proconio::{fastout, input};

const MOD: usize = 100000000;

#[fastout]
fn main() {
  input! {a:[usize]}
  let sum = a.into_iter().fold(0, |s, a| (s + a) % MOD);
  println!("{}", sum * (sum + MOD - 2) % MOD);
}
