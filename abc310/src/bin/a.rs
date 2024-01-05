#![allow(non_snake_case)]
use proconio::{fastout, input};

#[fastout]
fn main() {
  input! {n:usize,p:u32,q:u32,d:[u32;n]}
  println!("{}", p.min(q + d.iter().min().unwrap()));
}
