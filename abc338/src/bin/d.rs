#![allow(non_snake_case)]
use itertools::Itertools;
use proconio::{fastout, input, marker::Usize1};

#[fastout]
fn main() {
  input! {n:usize,x:[i64]}
  let mut l = 0;
  let mut s = vec![0; n];
  for (a, b) in x.into_iter().tuple_windows() {
    let p = (a - b).abs();
    l += p;
    s[a.max(b) as usize - 1] += n as i64 - 2 * p;
    s[a.min(b) as usize - 1] -= n as i64 - 2 * p;
  }
  let mut ans = l;
  for s in s.into_iter().rev() {
    l += s;
    ans = ans.min(l);
  }
  println!("{}", ans);
}
