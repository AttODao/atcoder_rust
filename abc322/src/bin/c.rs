#![allow(non_snake_case)]
use itertools::Itertools;
use proconio::{fastout, input, marker::Usize1};

#[fastout]
fn main() {
  input! {n:usize,m:usize,a:[Usize1;m]}

  let mut ans = vec![0; n];
  let mut j = m - 1;
  for i in (0..n).rev() {
    if j > 0 && a[j - 1] == i {
      j -= 1;
    }
    ans[i] = a[j] - i;
  }
  println!(
    "{}",
    ans.iter().map(usize::to_string).collect_vec().join("\n")
  );
}
