#![allow(non_snake_case)]
use itertools::Itertools;
use proconio::{input, source};

fn main() {
  let mut stdin = source::line::LineSource::new(std::io::BufReader::new(std::io::stdin()));
  macro_rules! input(($($tt:tt)*) => (proconio::input!(from &mut stdin, $($tt)*)));
  input! {n:usize,k:usize}

  let sumarm = (1..=k + 1)
    .map(|i| {
      println!("? {}", (1..=k + 1).filter(|&j| j != i).join(" "));
      input! {ret:u8}
      ret
    })
    .collect_vec();
  let suma = sumarm.iter().fold(0, |sum, x| sum ^ x);
  let mut a = sumarm.iter().map(|sumarm| sumarm ^ suma).collect_vec();
  let suma2 = suma ^ a[0] ^ a[1];
  for i in k + 2..=n {
    println!("? {} {}", (3..=k + 1).join(" "), i);
    input! {sum:u8}
    a.push(sum ^ suma2);
  }
  println!("! {}", a.iter().join(" "));
}
