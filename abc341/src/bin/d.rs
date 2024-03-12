#![allow(non_snake_case)]
use num_integer::gcd;
use proconio::{fastout, input};

#[fastout]
fn main() {
  input! {n:u128,m:u128,k:u128}
  let lcm = n * m / gcd(n, m);
  let (mut l, mut r) = (0, u64::MAX as u128);
  while r - l > 1 {
    let mid = (l + r) / 2;
    if mid / n + mid / m - mid / lcm * 2 < k {
      l = mid;
    } else {
      r = mid;
    }
  }
  println!("{}", r);
}
