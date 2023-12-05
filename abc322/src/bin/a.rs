#![allow(non_snake_case)]
use proconio::{fastout, input};

#[fastout]
fn main() {
  input! {n:usize,s:String}

  for i in 0..(n - 3) {
    if &s[i..(i + 3)] == "ABC" {
      println!("{}", i + 1);
      return;
    }
  }
  println!("-1");
}
