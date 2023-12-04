#![allow(non_snake_case)]
use proconio::{fastout, input, marker::Usize1};

#[fastout]
fn main() {
  input! {M:usize,D:usize,y:usize,m:Usize1,d:Usize1}

  let p = (y * M + m) * D + d + 1;

  println!("{} {} {}", p / (M * D), p / D % M + 1, p % D + 1);
}
