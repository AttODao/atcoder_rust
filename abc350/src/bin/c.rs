#![allow(non_snake_case)]
use itertools::Itertools;
use proconio::{fastout, input, marker::Usize1};

#[fastout]
fn main() {
  input! {n:usize,mut a:[Usize1;n]}
  let mut table = vec![0; n];
  for i in 1..n {
    table[a[i]] = i;
  }
  let mut ans = vec![];
  for i in 0..n - 1 {
    if i != a[i] {
      ans.push(format!("{} {}", i + 1, table[i] + 1));
      let tmp = a[i];
      a.swap(i, table[i]);
      table.swap(i, tmp);
    }
  }
  println!("{}\n{}", ans.len(), ans.iter().join("\n"))
}
