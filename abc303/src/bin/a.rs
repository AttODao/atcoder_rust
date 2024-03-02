#![allow(non_snake_case)]
use proconio::{fastout, input};

#[fastout]
fn main() {
  input! {_:usize,s:String,t:String}
  println!(
    "{}",
    if s.replace("1", "l").replace("0", "o") == t.replace("1", "l").replace("0", "o") {
      "Yes"
    } else {
      "No"
    }
  );
}
