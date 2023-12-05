use std::{cmp::Reverse, collections::HashSet};

use itertools::Itertools;
use proconio::{fastout, input, marker::Usize1};

#[fastout]
fn main() {
  input! {n:usize,m:usize,l:usize,mut a:[u64;n],mut b:[u64;m],cd:[(Usize1,Usize1);l]}

  let b = b
    .into_iter()
    .enumerate()
    .sorted_by_key(|&(_, b)| Reverse(b))
    .collect_vec();
  let cd = cd.into_iter().collect::<HashSet<(usize, usize)>>();
  let mut ans = 0;
  for (i, a) in a.into_iter().enumerate() {
    for &(j, b) in b.iter() {
      if !cd.contains(&(i, j)) {
        ans = ans.max(a + b);
        break;
      }
    }
  }
  println!("{}", ans);
}
