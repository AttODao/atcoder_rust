#![allow(non_snake_case)]
use proconio::{fastout, input};

#[fastout]
fn main() {
  input! {_:usize,x:usize,y:usize,z:usize}
  println!(
    "{}",
    if x.min(y) < z && z < x.max(y) {
      "Yes"
    } else {
      "No"
    }
  );
}
