#![allow(non_snake_case)]
use std::collections::BTreeSet;

use proconio::{fastout, input, marker::Usize1};

#[fastout]
fn main() {
  input! {n:usize,k:usize,p:[Usize1;n]}
  let pos = p.into_iter().enumerate().fold(vec![0; n], |mut f, (i, p)| {
    f[p] = i;
    f
  });
  let mut set = pos[0..k].iter().collect::<BTreeSet<_>>();
  let mut ans = *set.last().unwrap() - *set.first().unwrap();
  for i in k..n {
    set.insert(&pos[i]);
    set.remove(&pos[i - k]);
    ans = ans.min(*set.last().unwrap() - *set.first().unwrap());
  }
  println!("{}", ans);
}
