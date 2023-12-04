use std::{cmp::Reverse, collections::BinaryHeap};

use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
  input! {n:usize,k:usize,s:Chars}

  let mut pq = BinaryHeap::new();
  for i in 0..(n - k) {
    pq.push(Reverse((s[i], i)));
  }
  let mut index = 0;
  let mut ans = vec![];
  for i in (n - k)..n {
    pq.push(Reverse((s[i], i)));
    while let Some(Reverse((c, j))) = pq.pop() {
      if j >= index {
        ans.push(c);
        index = j;
        break;
      }
    }
  }
  println!("{}", ans.into_iter().collect::<String>());
}
