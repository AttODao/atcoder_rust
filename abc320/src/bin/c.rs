#![allow(non_snake_case)]
use itertools::Itertools;
use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
  input! {m:usize,s:[Chars;3]}

  let mut ans = usize::MAX;
  for t in (0..3 * m).permutations(3) {
    if s.iter().zip(t.iter()).map(|(s, &t)| s[t % m]).all_equal() {
      ans = ans.min(t.into_iter().max().unwrap());
    }
  }
  println!("{}", if ans < usize::MAX { ans as isize } else { -1 });
}
