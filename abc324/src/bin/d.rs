use itertools::Itertools;
use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
  input! {n:usize,mut s:Chars}

  s.sort();
  let mut ans = 0i64;
  for i in 0..10000000i64 {
    let p = format!("{:0>1$}", i * i, n).chars().sorted().collect_vec();
    if p == s {
      ans += 1;
    }
  }
  println!("{}", ans);
}
