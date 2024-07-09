#![allow(non_snake_case)]
use itertools::Itertools;
use proconio::{fastout, input, marker::Usize1};

#[fastout]
fn main() {
  input! {n:usize,m:usize,k:u32,ar:[([Usize1],char);m]}
  let ar = ar
    .into_iter()
    .map(|(a, r)| (a.into_iter().map(|a| 1 << a).sum::<usize>(), r == 'o'))
    .collect_vec();
  println!(
    "{}",
    (0..1 << n)
      .filter(|i| ar.iter().all(|(a, r)| ((i & a).count_ones() >= k) == *r))
      .count()
  );
}
