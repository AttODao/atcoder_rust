#![allow(non_snake_case)]
use proconio::{fastout, input, marker::Usize1};

#[fastout]
fn main() {
  input! {n:usize,ab:[(Usize1,Usize1);n]}
  let mut s = vec![(0, false); 2 * n];
  for (i, (a, b)) in ab.into_iter().enumerate() {
    s[a.min(b)] = (i, true);
    s[a.max(b)] = (i, false);
  }
  let mut stack = vec![];
  for (i, t) in s {
    if t {
      stack.push(i);
    } else {
      if i != stack.pop().unwrap() {
        println!("Yes");
        return;
      }
    }
  }
  println!("No");
}
