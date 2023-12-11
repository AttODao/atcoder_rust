#![allow(non_snake_case)]
use proconio::{fastout, input};

#[fastout]
fn main() {
  input! {n:usize,s:u64,k:u64,pq:[(u64,u64);n]}

  let sum = pq.into_iter().map(|(p, q)| p * q).sum::<u64>();
  println!("{}", sum + (sum < s) as u64 * k);
}
