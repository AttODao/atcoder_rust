#![allow(non_snake_case)]
use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
  input! {_:usize,m:i32,mut s:Chars}

  s.push('0');
  let mut ans = 0;
  let (mut e1, mut e2) = (0, 0);
  for c in s {
    match c {
      '0' => {
        ans = ans.max(0.max(e1 - m) + e2);
        e1 = 0;
        e2 = 0;
      }
      '1' => {
        e1 += 1;
      }
      '2' => {
        e2 += 1;
      }
      _ => (),
    }
  }
  println!("{}", ans);
}
