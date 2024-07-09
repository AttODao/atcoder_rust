#![allow(non_snake_case)]
use ac_library::ModInt998244353;
use proconio::{fastout, input};

#[fastout]
fn main() {
  input! {n:u64}
  let mut b = 1;
  while b <= n {
    b *= 10;
  }
  let b = ModInt998244353::new(b);
  println!("{}", (b.pow(n) - 1) / (b - 1) * n);
}
