#![allow(non_snake_case)]
use itertools::izip;
use proconio::{fastout, input, marker::Usize1};

#[fastout]
fn main() {
  input! {n:usize, a:[Usize1;n], w:[usize;n]}
  let mut sum = vec![0; n];
  let mut max = vec![0; n];
  for (a, w) in izip!(a, w) {
    sum[a] += w;
    max[a] = max[a].max(w);
  }
  println!(
    "{}",
    izip!(sum, max).map(|(sum, max)| sum - max).sum::<usize>()
  );
}
