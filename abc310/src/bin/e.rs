#![allow(non_snake_case)]
use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
  input! {_:usize,a:Chars}
  let mut zo = (0, 0);
  let mut ans: u64 = 0;
  for a in a {
    if a == '0' {
      zo = (1, zo.0 + zo.1);
    } else {
      zo = (zo.1, zo.0 + 1);
    }
    ans += zo.1;
  }
  println!("{}", ans);
}
