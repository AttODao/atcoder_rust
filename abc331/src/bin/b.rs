use itertools::iproduct;
use proconio::{fastout, input};

#[fastout]
fn main() {
  input! {n:usize,s:usize,m:usize,l:usize}

  let mut ans = usize::MAX;
  for (i, j, k) in iproduct!(0..=n, 0..=n, 0..=n) {
    if 6 * i + 8 * j + 12 * k >= n {
      ans = ans.min(i * s + j * m + k * l);
    }
  }
  println!("{}", ans);
}
