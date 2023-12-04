use itertools::Itertools;
use proconio::{fastout, input};

#[fastout]
fn main() {
  input! {s:String}
  let ans = s.chars().sorted().collect::<String>();
  print!("{}", ans);
}
