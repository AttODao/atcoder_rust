#![allow(non_snake_case)]
use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
  input! {s:Chars}
  let mut cnt = vec![0; 26];
  let n = s.len();
  let mut flag = false;
  for s in s {
    cnt[s as usize - 'a' as usize] += 1;
    if cnt[s as usize - 'a' as usize] > 1 {
      flag = true;
    }
  }
  println!(
    "{}",
    (n * n - cnt.iter().map(|&c| c * c).sum::<usize>()) / 2 + flag as usize
  );
}
