use std::{cmp::Reverse, collections::BinaryHeap};

use proconio::{fastout, input};

#[fastout]
fn main() {
  input! {n:usize,mut td:[(i128,i128);n]}

  let mut events = vec![];
  for &(t, d) in td.iter() {
    events.push((t, t + d));
  }
  events.sort();

  let mut bh = BinaryHeap::new();
  let mut time = 0;
  let mut index = 0;
  let mut ans = 0;
  loop {
    if bh.is_empty() {
      if index == n {
        break;
      }
      time = events[index].0;
    }
    while index < n && events[index].0 == time {
      bh.push(Reverse(events[index].1));
      index += 1;
    }
    while let Some(&Reverse(p)) = bh.peek() {
      if p < time {
        bh.pop();
      } else {
        break;
      }
    }
    if !bh.is_empty() {
      bh.pop();
      ans += 1;
    }
    time += 1;
  }
  println!("{}", ans);
}
