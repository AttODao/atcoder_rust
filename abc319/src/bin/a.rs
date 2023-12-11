#![allow(non_snake_case)]
use std::collections::HashMap;

use itertools::Itertools;
use proconio::{fastout, input};

#[fastout]
fn main() {
  input! {s:String}

  let r = r"tourist 3858
ksun48 3679
Benq 3658
Um_nik 3648
apiad 3638
Stonefeang 3630
ecnerwala 3613
mnbvmar 3555
newbiedmy 3516
semiexp 3481"
    .to_string()
    .split("\n")
    .map(|s| {
      let (name, rating) = s.split(" ").collect_tuple().unwrap();
      (name, rating.parse::<u32>().unwrap())
    })
    .collect::<HashMap<&str, u32>>()[&s[..]];
  println!("{}", r);
}
