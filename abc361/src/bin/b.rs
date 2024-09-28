#![allow(non_snake_case)]
use itertools::izip;
use proconio::{fastout, input};

#[fastout]
fn main() {
  input! {p11:[u32;3],p12:[u32;3],p21:[u32;3],p22:[u32;3]}
  if izip!(p11, p12, p21, p22).all(|(x11, x12, x21, x22)| x12 > x21 && x11 < x22) {
    println!("Yes")
  } else {
    println!("No")
  }
}
