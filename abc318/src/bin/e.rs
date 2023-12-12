#![allow(non_snake_case)]
use itertools::Itertools;
use proconio::{fastout, input, marker::Usize1};

#[fastout]
fn main() {
  input! {n:usize,a:[Usize1;n]}

  let mut sumlr = 0;
  let mut l = vec![0; n];
  let mut r = vec![0; n];
  for &a in a[1..].iter() {
    r[a] += 1;
  }
  let mut ans: u64 = 0;
  for (&a1, &a0) in a.iter().tuple_windows() {
    sumlr += r[a1];
    sumlr -= l[a0];
    if a1 == a0 {
      sumlr -= 1;
    }
    l[a1] += 1;
    r[a0] -= 1;
    ans += sumlr - l[a0] * r[a0];
  }
  println!("{}", ans);
}
