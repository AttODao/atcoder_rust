#![allow(non_snake_case)]
use proconio::{fastout, input};

#[fastout]
fn main() {
  input! {n:usize,a:[usize;n],lr:[(usize,usize)]}
  let mut fa = vec![0; n];
  for i in 0..n / 2 {
    fa[2 * i + 1] = fa[2 * i];
    fa[2 * i + 2] = fa[2 * i + 1] + (a[2 * i + 2] - a[2 * i + 1]);
  }
  let f = |x: usize| {
    let (mut l, mut r) = (0, n);
    while r - l > 1 {
      let mid = (l + r) / 2;
      if a[mid] < x {
        l = mid;
      } else {
        r = mid;
      }
    }
    fa[l] + (x - a[l]) * (fa[r] - fa[l]) / (a[r] - a[l])
  };
  for (l, r) in lr {
    println!("{}", f(r) - f(l));
  }
}
