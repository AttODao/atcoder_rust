#![allow(non_snake_case)]
use std::collections::VecDeque;

use proconio::{fastout, input};

#[fastout]
fn main() {
  input! {n:usize,s:u64,a:[u64;n]}

  let mut p = vec![n; n];
  let mut sum = 0;
  let mut queue = VecDeque::new();
  for (i, &a) in a.iter().enumerate() {
    queue.push_back((sum, i));
    sum += a;
    while sum - queue.front().unwrap().0 > s {
      let j = queue.pop_front().unwrap().1;
      p[j] = i;
    }
  }
  let mut dp = vec![0; n + 1];
  for i in (0..n).rev() {
    dp[i] = dp[p[i]] + n - i;
  }
  println!("{}", dp.iter().sum::<usize>());
}
