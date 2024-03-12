#![allow(non_snake_case)]
use proconio::{fastout, input, marker::Usize1};

#[fastout]
fn main() {
  input! {n:usize,p:[Usize1;n],ab:[(Usize1,Usize1)]}
  let mut x = vec![0; n];
  for i in 0..n {
    x[p[i]] = i;
  }
  for (a, b) in ab {
    println!("{}", if x[a] < x[b] { a } else { b } + 1);
  }
}
