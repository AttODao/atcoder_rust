use std::collections::BTreeMap;

use itertools::Itertools;
use proconio::{fastout, input};

#[fastout]
fn main() {
  input! {n:usize,a:[usize;n]}

  let mut ans = vec![0; n];
  let mut sum = 0;
  let mut set = BTreeMap::<usize, Vec<usize>>::new();
  for (i, a) in a.into_iter().enumerate() {
    set.entry(a).or_default().push(i);
  }
  for (a, i) in set.into_iter().rev() {
    for &i in i.iter() {
      ans[i] = sum;
    }
    sum += a * i.len();
  }
  println!("{}", ans.into_iter().join(" "));
}
