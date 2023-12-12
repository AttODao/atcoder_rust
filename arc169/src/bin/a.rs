#![allow(non_snake_case)]
use itertools::izip;
use proconio::{fastout, input, marker::Usize1};

#[fastout]
fn main() {
  input! {n:usize,a:[i64;n],mut p:[Usize1;n-1]}

  let mut d = vec![0; n];
  for (i, &p) in p.iter().enumerate() {
    d[i + 1] = d[p] + 1;
  }
  let mut sum = vec![0; n];
  for (a, d) in izip!(a, d) {
    sum[d] += a;
  }
  for &sum in sum.iter().rev() {
    match sum.cmp(&0) {
      std::cmp::Ordering::Less => {
        println!("-");
        return;
      }
      std::cmp::Ordering::Equal => (),
      std::cmp::Ordering::Greater => {
        println!("+");
        return;
      }
    }
  }
  println!("0");
}
