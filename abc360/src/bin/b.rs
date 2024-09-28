#![allow(non_snake_case)]
use itertools::Itertools;
use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
  input! {s:Chars,t:Chars}
  let t = t.iter().collect_vec();
  for w in 1..s.len() {
    for c in 0..w {
      let v = s.iter().skip(c).step_by(w).collect_vec();
      if v == t {
        println!("Yes");
        return;
      }
    }
  }
  println!("No");
}
