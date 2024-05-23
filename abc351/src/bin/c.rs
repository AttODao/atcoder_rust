#![allow(non_snake_case)]
use proconio::{fastout, input};

#[fastout]
fn main() {
  input! {a:[usize]}
  let mut table = vec![];
  for mut a in a {
    while table.last() == Some(&a) {
      table.pop();
      a += 1;
    }
    table.push(a);
  }
  println!("{}", table.len());
}
