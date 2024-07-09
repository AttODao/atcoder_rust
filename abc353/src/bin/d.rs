#![allow(non_snake_case)]
use ac_library::ModInt998244353;
use proconio::{fastout, input};

#[fastout]
fn main() {
  input! {a:[usize]}
  let mut s = ModInt998244353::new(0);
  let mut d = ModInt998244353::new(0);
  let mut ans = ModInt998244353::new(0);
  for a in a.into_iter().rev() {
    ans += ModInt998244353::new(a) * d + s;
    d += ModInt998244353::new(10).pow(a.to_string().len() as u64);
    s += a;
  }
  println!("{}", ans);
}
