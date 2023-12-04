use ac_library::ModInt998244353;
use proconio::{fastout, input};

#[fastout]
fn main() {
  input! {n:usize,a:[usize;n]}

  let mut p = ModInt998244353::new(1) / n;
  let q = ModInt998244353::new(n + 1) / n;
  let mut ans = ModInt998244353::new(0);
  for a in a {
    ans += p * a;
    p *= q;
  }
  println!("{}", ans);
}
