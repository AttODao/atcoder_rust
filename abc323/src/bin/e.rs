use ac_library::ModInt998244353;
use proconio::{fastout, input};

#[fastout]
fn main() {
  input! {n:usize,x:usize,t:[usize;n]}

  let mut p = vec![ModInt998244353::new(0); x + 1];
  p[0] += 1;
  let invn = ModInt998244353::new(1) / n;
  for u in 1..=x {
    for &t in &t {
      if u >= t {
        let q = p[u - t];
        p[u] += q;
      }
    }
    p[u] *= invn;
  }
  p.reverse();
  println!(
    "{}",
    p[0..t[0].min(x + 1)].iter().sum::<ModInt998244353>() * invn
  );
}
