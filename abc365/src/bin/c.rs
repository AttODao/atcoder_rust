#![allow(non_snake_case)]
use proconio::{fastout, input};

const BIG: u64 = 1 << 60;

#[fastout]
fn main() {
  input! {n:usize,m:u64,a:[u64;n]}
  let (mut ok, mut ng) = (0, BIG);
  while ng - ok > 1 {
    let mid = (ok + ng) / 2;
    if a.iter().map(|&a| a.min(mid)).sum::<u64>() <= m {
      ok = mid;
    } else {
      ng = mid;
    }
  }
  if ng == BIG {
    println!("infinite");
  } else {
    println!("{}", ok);
  }
}
