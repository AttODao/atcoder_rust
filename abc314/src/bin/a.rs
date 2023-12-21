#![allow(non_snake_case)]
use proconio::{fastout, input};

#[fastout]
fn main() {
  input! {n:usize}
  println!("{}",&"3.1415926535897932384626433832795028841971693993751058209749445923078164062862089986280348253421170679".to_string()[..n+2]);
}
