#![allow(non_snake_case)]
use im_rc::HashMap;
use proconio::{fastout, input};

#[fastout]
fn main() {
  input! {_:usize,_:usize,pq:[(usize,usize)],a:[usize],b:[usize]}
  let mut map = HashMap::<_, usize>::new();
  for (p, q) in pq {
    *map
      .entry((a.partition_point(|&a| a < p), b.partition_point(|&b| b < q)))
      .or_default() += 1;
  }
  println!(
    "{} {}",
    if map.len() < (a.len() + 1) * (b.len() + 1) {
      0
    } else {
      *map.values().min().unwrap()
    },
    map.values().max().unwrap()
  )
}
