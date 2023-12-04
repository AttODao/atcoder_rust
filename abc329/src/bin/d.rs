use std::cmp::Reverse;

use proconio::{fastout, input, marker::Usize1};

#[fastout]
fn main() {
  input! {n:usize,m:usize,a:[Usize1;m]}

  let mut votes = vec![0; n];
  let mut elected = 0;
  for a in a {
    votes[a] += 1;
    if (votes[a], Reverse(a)) > (votes[elected], Reverse(elected)) {
      elected = a;
    }
    println!("{}", elected + 1);
  }
}
