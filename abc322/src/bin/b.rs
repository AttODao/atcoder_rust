#![allow(non_snake_case)]
use proconio::{fastout, input};

#[fastout]
fn main() {
  input! {_:usize,_:usize,s:String,t:String}
  println!(
    "{}",
    if t.starts_with(&s) { 0 } else { 2 } + if t.ends_with(&s) { 0 } else { 1 }
  );
}
