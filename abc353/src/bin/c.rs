#![allow(non_snake_case)]
use proconio::{fastout, input};

const M: usize = 100000000;

#[fastout]
fn main() {
  input! {n:usize, a:[usize;n]}
  let mut cnt = vec![0; M];
  for &a in &a {
    cnt[a] += 1;
  }
  for i in (0..M - 1).rev() {
    cnt[i] += cnt[i + 1];
  }
  let mut c = 0;
  for &a in &a {
    c += cnt[M - a];
    if a * 2 >= M {
      c -= 1;
    }
  }
  let sum = a.iter().sum::<usize>();
  println!("{}", sum * (n - 1) - M * c / 2);
}
