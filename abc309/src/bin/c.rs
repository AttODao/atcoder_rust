#![allow(non_snake_case)]
use proconio::{fastout, input};

#[fastout]
fn main() {
  input! {n:usize,k:u64,ab:[(u64,u64);n]}
  let (mut ng, mut ok) = (0, 1 << 30);
  while ok - ng > 1 {
    let mid = (ng + ok) / 2;
    if ab
      .iter()
      .filter_map(|&(a, b)| if a >= mid { Some(b) } else { None })
      .sum::<u64>()
      <= k
    {
      ok = mid;
    } else {
      ng = mid;
    }
  }
  println!("{}", ok);
}
