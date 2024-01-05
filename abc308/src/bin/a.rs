#![allow(non_snake_case)]
use itertools::Itertools;
use proconio::{fastout, input};

#[fastout]
fn main() {
  input! {s:[u32;8]}
  println!(
    "{}",
    if s.iter().tuple_windows().all(|(a, b)| a <= b)
      && s[0] >= 100
      && s[7] <= 675
      && s.iter().all(|s| s % 25 == 0)
    {
      "Yes"
    } else {
      "No"
    }
  );
}
