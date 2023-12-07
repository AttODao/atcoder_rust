#![allow(non_snake_case)]
use itertools::Itertools;
use proconio::{fastout, input, marker::Usize1};

fn dfs(x: u64) -> Vec<u64> {
  let mut ret = vec![x];
  for next in (x * 10)..(x * 10 + x % 10) {
    ret.append(&mut dfs(next));
  }
  ret
}

#[fastout]
fn main() {
  input! {k:Usize1}

  let nums = (1..10).map(|i| dfs(i)).flatten().sorted().collect_vec();
  println!("{}", nums[k]);
}
