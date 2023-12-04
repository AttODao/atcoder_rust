use itertools::Itertools;
use proconio::{fastout, input};

#[fastout]
fn main() {
  input! {n:usize,a:[i32;n]}
  println!("{}", if a.iter().all_equal() { "Yes" } else { "No" });
}
