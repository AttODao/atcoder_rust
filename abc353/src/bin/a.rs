#![allow(non_snake_case)]
use proconio::{fastout, input};

#[fastout]
fn main() {
  input! {h:[usize]}
  if let Some((i, _)) = h[1..].iter().enumerate().find(|&(_, &l)| l > h[0]) {
    println!("{}", i + 2);
  } else {
    println!("{}", -1);
  }
}
