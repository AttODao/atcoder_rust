#![allow(non_snake_case)]
use itertools::Itertools;
use proconio::{fastout, input, input_interactive};

// #[fastout]
fn main() {
  input_interactive! {n:usize}
  let m = n.next_power_of_two().trailing_zeros() as usize;
  let mut a = vec![vec![]; m];
  for i in 0..n {
    for j in 0..m {
      if (i >> j) & 1 == 1 {
        a[j].push(i + 1);
      }
    }
  }
  println!("{}", m);
  for a in a.into_iter().rev() {
    println!("{} {}", a.len(), a.iter().join(" "));
  }
  input_interactive! {s:String}
  println!("{}", usize::from_str_radix(&s, 2).unwrap() + 1);
}
