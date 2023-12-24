#![allow(non_snake_case)]
use proconio::{fastout, input};

#[fastout]
fn main() {
  input! {B:u32,G:u32}
  println!("{}", if B > G { "Bat" } else { "Glove" });
}
