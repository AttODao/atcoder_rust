#![allow(non_snake_case)]
use ac_library::FenwickTree;
use itertools::Itertools;
use proconio::{fastout, input};

#[fastout]
fn main() {
  input! {n:usize,m:usize,a:[i64;n],b:[usize;m]}
  let mut ft = FenwickTree::new(n, 0);
  ft.add(0, a[0]);
  for i in 1..n {
    ft.add(i, a[i] - a[i - 1]);
  }
  for b in b {
    let ball = ft.accum(b + 1);
    ft.add(b, -ball);
    if b + 1 < n {
      ft.add(b + 1, ball);
    }
    let l = (b + 1) % n;
    let r = (b + ball as usize + 1) % n;
    ft.add(0, ball / n as i64);
    ft.add(l, 1);
    ft.add(r, -1);
    if l > r {
      ft.add(0, 1);
    }
  }
  println!("{}", (0..n).map(|i| ft.accum(i + 1)).join(" "));
}
