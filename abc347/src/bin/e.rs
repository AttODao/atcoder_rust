#![allow(non_snake_case)]
use itertools::Itertools;
use proconio::{fastout, input, marker::Usize1};

#[fastout]
fn main() {
  input! {n:usize,q:usize,x:[Usize1;q]}
  let mut s = vec![None; n];
  let mut slen: u64 = 0;
  let mut sum = 0;
  let mut a = vec![0; n];
  for x in x {
    if let Some(addsum) = s[x] {
      s[x] = None;
      a[x] += sum - addsum;
      slen -= 1;
      sum += slen;
    } else {
      s[x] = Some(sum);
      slen += 1;
      sum += slen;
    }
  }
  for i in 0..n {
    if let Some(addsum) = s[i] {
      a[i] += sum - addsum;
    }
  }
  println!("{}", a.into_iter().join(" "));
}
