use ac_library::floor_sum;
use proconio::{fastout, input};

#[fastout]
fn main() {
  input! {t:usize,tasks:[(i64,i64,i64,i64);t]}

  for (n, m, a, b) in tasks {
    println!("{}", floor_sum(n, m, a, b));
  }
}
