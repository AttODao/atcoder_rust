use std::cmp::Reverse;

use ac_library::FenwickTree;
use proconio::{fastout, input, marker::Usize1};

#[fastout]
fn main() {
  input! {n:usize,m:usize,mut lr:[(Usize1,Usize1);m]}

  let mut ans = 0;
  let mut ft = FenwickTree::<i64>::new(n, 0);
  lr.sort_by_key(|&(l, r)| (r, Reverse(l)));
  for (l, r) in lr {
    ans += ft.sum(..l);
    ft.add(l, 1);
    ft.add(r - 1, -1);
  }
  println!("{}", ans);
}
