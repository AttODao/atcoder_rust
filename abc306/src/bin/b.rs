#![allow(non_snake_case)]
use proconio::{fastout, input};

#[fastout]
fn main() {
  input! {a:[char;64]}
  println!(
    "{}",
    usize::from_str_radix(&a.iter().rev().collect::<String>(), 2).unwrap()
  );
}
