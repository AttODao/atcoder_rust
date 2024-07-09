#![allow(non_snake_case)]
use itertools::{izip, Itertools};
use proconio::{fastout, input};

#[fastout]
fn main() {
  input! {n:usize,m:usize,a:[usize;m],x:[[usize;m];n]}
  if izip!(
    a,
    x.into_iter().fold(vec![0; m], |s, x| {
      izip!(s, x).map(|(s, x)| s + x).collect_vec()
    })
  )
  .all(|(a, s)| a <= s)
  {
    println!("Yes");
  } else {
    println!("No");
  }
}
