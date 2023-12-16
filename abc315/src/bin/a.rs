#![allow(non_snake_case)]
use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
  input! {s:Chars}
  println!(
    "{}",
    s.into_iter()
      .filter(|c| !"aiueo".contains(*c))
      .collect::<String>()
  )
}
