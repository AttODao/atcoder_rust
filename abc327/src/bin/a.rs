use itertools::Itertools;
use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
  input! {_:usize,s:Chars}
  for (&a, &b) in s.iter().tuple_windows() {
    if a == 'a' && b == 'b' || a == 'b' && b == 'a' {
      println!("Yes");
      return;
    }
  }
  println!("No");
}
