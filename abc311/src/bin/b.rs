#![allow(non_snake_case)]
use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
  input! {n:usize,d:usize,s:[Chars;n]}
  let mut ans = 0;
  let mut cnt = 0;
  for i in 0..d {
    if s.iter().all(|s| s[i] == 'o') {
      cnt += 1;
      ans = ans.max(cnt);
    } else {
      cnt = 0;
    }
  }
  println!("{}", ans);
}
