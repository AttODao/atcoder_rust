#![allow(non_snake_case)]
use proconio::input;

fn main() {
  input! {s:String,t:String}
  let mut ti = t.chars().enumerate();
  s.chars()
    .for_each(|s| print!("{} ", ti.find(|&(_, t)| s == t).unwrap().0 + 1));
}
