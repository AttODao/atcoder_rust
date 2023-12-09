use itertools::izip;
use proconio::{fastout, input};

#[fastout]
fn main() {
  input! {n:usize,mut k:i64,a:[i64;n],b:[i64;n]}

  for (a, b) in izip!(a, b) {
    k -= (a - b).abs();
  }
  println!("{}", if k >= 0 && k & 1 == 0 { "Yes" } else { "No" });
}
