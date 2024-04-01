#![allow(non_snake_case)]
use proconio::{fastout, input};

#[fastout]
fn main() {
  input! {s:String}
  println!("{}", regex::Regex::new(r"\|.*\|").unwrap().replace(&s, ""));
}
