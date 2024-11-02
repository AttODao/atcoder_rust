#![allow(non_snake_case)]
use std::{cmp::Ordering, usize};

use itertools::izip;
use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
  input! {mut s:Chars,t:Chars}
  let mut table = vec![];
  for (i, (&s, &t)) in izip!(&s, &t).enumerate() {
    match s.cmp(&t) {
      Ordering::Less => table.push(usize::MAX - i),
      Ordering::Greater => table.push(i),
      Ordering::Equal => (),
    }
  }
  println!("{}", table.len());
  table.sort();
  for mut i in table {
    i = if i > usize::MAX / 2 {
      usize::MAX - i
    } else {
      i
    };
    s[i] = t[i];
    println!("{}", s.iter().collect::<String>());
  }
}
