use im_rc::HashSet;
use proconio::{fastout, input};

#[fastout]
fn main() {
  input! {s:[String;3]}

  let s: HashSet<String> = s.into();
  for c in ["ABC", "ARC", "AGC", "AHC"] {
    if !s.contains(&c.to_string()) {
      println!("{}", c);
      return;
    }
  }
}
