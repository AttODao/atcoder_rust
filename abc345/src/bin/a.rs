#![allow(non_snake_case)]
use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
  input! {s:Chars}
  println!(
    "{}",
    if s.first().unwrap() == &'<'
      && s.last().unwrap() == &'>'
      && s[1..s.len() - 1].iter().all(|c| c == &'=')
    {
      "Yes"
    } else {
      "No"
    }
  );
}
