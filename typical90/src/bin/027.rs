use im_rc::HashSet;
use itertools::Itertools;
use proconio::{fastout, input};

#[fastout]
fn main() {
  input! {n:usize,s:[String;n]}
  let mut set = HashSet::new();
  println!(
    "{}",
    (1..=n).filter(|i| set.insert(&s[i - 1]) == None).join("\n")
  );
}
