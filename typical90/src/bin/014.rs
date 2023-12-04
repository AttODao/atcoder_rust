use itertools::zip;
use proconio::{fastout, input};

#[fastout]
fn main() {
  input! {n:usize,mut a:[i64;n],mut b:[i64;n]}

  a.sort();
  b.sort();
  println!(
    "{}",
    zip(a.iter(), b.iter())
      .map(|(a, b)| (a - b).abs())
      .sum::<i64>()
  );
}
