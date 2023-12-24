#![allow(non_snake_case)]
use proconio::{fastout, input};

#[fastout]
fn main() {
  input! {s:String}

  if ["ACE", "BDF", "CEG", "DFA", "EGB", "FAC", "GBD"].contains(&&s[..]) {
    println!("Yes");
  } else {
    println!("No");
  }
}
