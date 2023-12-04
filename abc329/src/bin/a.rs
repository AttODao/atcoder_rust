use itertools::Itertools;
use proconio::{input, fastout, marker::Chars};

#[fastout]
fn main() {
  input! {s:Chars}
  println!("{}",s.iter().join(" "));
}
