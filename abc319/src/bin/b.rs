#![allow(non_snake_case)]
use proconio::{fastout, input};

#[fastout]
fn main() {
  input! {n:usize}

  for i in 0..=n {
    let mut s = "-".to_string();
    for j in 1..10 {
      if n % j == 0 && i % (n / j) == 0 {
        s = j.to_string();
        break;
      }
    }
    print!("{}", s);
  }
}
