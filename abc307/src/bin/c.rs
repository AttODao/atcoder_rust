#![allow(non_snake_case)]
use num_bigint::BigInt;
use num_traits::Zero;
use proconio::{fastout, input, marker::Chars};

fn input() -> BigInt {
  input! {h:usize,_:usize,s:[Chars;h]}
  let s = s.into_iter().fold(BigInt::zero(), |acc, s| {
    (acc << 20)
      | s.into_iter().fold(BigInt::zero(), |acc, c| {
        (acc << 1) | BigInt::from((c == '#') as u8)
      })
  });
  s.clone() / (s.clone() & -s)
}

#[fastout]
fn main() {
  let (a, b, x) = (input(), input(), input());
  println!(
    "{}",
    if (0..200).any(|i| a.clone() | (b.clone() << i) == x || (a.clone() << i) | b.clone() == x) {
      "Yes"
    } else {
      "No"
    }
  );
}
