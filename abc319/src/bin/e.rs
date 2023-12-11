#![allow(non_snake_case)]
use itertools::Itertools;
use proconio::{fastout, input};

#[fastout]
fn main() {
  input! {n:usize,x:usize,y:usize,pt:[(usize,usize);n-1],q:usize,q:[usize;q]}

  let times = (0..840)
    .map(|start| {
      let mut time = start;
      time += x;
      for &(p, t) in &pt {
        time = (time + p - 1) / p * p + t;
      }
      time + y - start
    })
    .collect_vec();
  println!("{}", q.iter().map(|&q| q + times[q % 840]).join("\n"));
}
