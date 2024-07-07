#![allow(non_snake_case)]
use ac_library::ModInt998244353;
use proconio::{fastout, input};

#[fastout]
fn main() {
  input! {n:usize,m:usize}
  let mut ans = ModInt998244353::new(0);
  for i in 0..60 {
    if (m >> i) & 1 == 1 {
      let (l, b, r) = (n & !((1 << i + 1) - 1), (n >> i) & 1, n & (1 << i) - 1);
      ans += l / 2;
      if b == 1 {
        ans += r + 1;
      }
    }
  }
  println!("{ans}");
}
