#![allow(non_snake_case)]
use im_rc::HashSet;
use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
  input! {s:[Chars]}
  println!(
    "{}",
    s.into_iter()
      .map(|s| {
        let mut rev = s.clone();
        rev.reverse();
        s.min(rev)
      })
      .collect::<HashSet<Vec<char>>>()
      .len()
  );
}
