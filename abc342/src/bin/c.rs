#![allow(non_snake_case)]
use itertools::Itertools;
use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
  input! {_:usize,s:Chars,cd:[(char,char)]}
  let mut repl = ('a'..='z').collect_vec();
  for (c, d) in cd {
    for i in 0..26 {
      if repl[i] == c {
        repl[i] = d;
      }
    }
  }
  println!(
    "{}",
    s.iter()
      .map(|&s| repl[s as usize - 'a' as usize])
      .collect::<String>()
  );
}
