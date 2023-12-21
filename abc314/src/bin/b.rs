#![allow(non_snake_case)]
use itertools::Itertools;
use proconio::{fastout, input};

#[fastout]
fn main() {
  input! {a:[[usize]],x:usize}
  let p = a
    .into_iter()
    .enumerate()
    .filter_map(|(i, a)| {
      if a.contains(&x) {
        Some((a.len(), i))
      } else {
        None
      }
    })
    .collect_vec();
  let m = p.iter().min();
  let ans = p
    .iter()
    .filter_map(|(l, i)| {
      if Some(l) == m.map(|(l, _)| l) {
        Some(i + 1)
      } else {
        None
      }
    })
    .collect_vec();
  println!("{}\n{}", ans.len(), ans.iter().join(" "));
}
