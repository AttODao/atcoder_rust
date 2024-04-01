#![allow(non_snake_case)]
use std::collections::HashSet;

use itertools::{iproduct, Itertools};
use proconio::{fastout, input};

#[fastout]
fn main() {
  input! {a:[u32],b:[u32],c:[u32],x:[u32]}
  let mut set = HashSet::new();
  for (a, b, c) in iproduct!(a, b, c) {
    set.insert(a + b + c);
  }
  println!(
    "{}",
    x.into_iter()
      .map(|x| if set.contains(&x) { "Yes" } else { "No" })
      .join("\n")
  );
}
