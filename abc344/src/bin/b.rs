#![allow(non_snake_case)]
use proconio::{fastout, input};

fn f() {
  input! {a:u32}
  if a > 0 {
    f();
  }
  println!("{a}");
}

#[fastout]
fn main() {
  f();
}
