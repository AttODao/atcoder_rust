#![allow(non_snake_case)]
use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
  input! {_:usize,s:Chars}
  let mut dp = vec![0; 3];
  for s in s {
    let mut _dp = dp.clone();
    let h = match s {
      'R' => 0,
      'P' => 1,
      'S' => 2,
      _ => unreachable!(),
    };
    for i in 0..2 {
      _dp[(h + i) % 3] = dp[(h + i + 1) % 3].max(dp[(h + i + 2) % 3]) + i;
    }
    _dp[(h + 2) % 3] = 0;
    dp = _dp;
  }
  println!("{}", dp.iter().max().unwrap());
}
