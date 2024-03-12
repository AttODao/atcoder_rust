#![allow(non_snake_case)]
use proconio::{fastout, input};

#[fastout]
fn main() {
  input! {n:u64}
  let ulog = n.next_power_of_two().trailing_zeros() as u64;
  println!("{}", n * (ulog + 1) - (1 << ulog));
}
