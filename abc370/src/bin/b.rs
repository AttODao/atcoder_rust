#![allow(non_snake_case)]
use proconio::{fastout, input, marker::Usize1};

#[fastout]
fn main() {
  input! {n:usize}
  let mut a = vec![];
  for i in 0..n {
    input! {na:[Usize1;i+1]}
    a.push(na);
  }
  let mut e = 0;
  for i in 0..n {
    e = a[e.max(i)][e.min(i)];
  }
  println!("{}", e + 1)
}
