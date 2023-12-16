#![allow(non_snake_case)]
use proconio::{fastout, input};

#[fastout]
fn main() {
  input! {n:usize,h:i64,x:i64,p:[i64;n]}

  let target = x - h;
  let (mut ng, mut ok) = (0, n);
  while ok - ng > 1 {
    let mid = (ng + ok) / 2;
    if p[mid - 1] >= target {
      ok = mid;
    } else {
      ng = mid;
    }
  }
  println!("{}", ok);
}
