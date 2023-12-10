#![allow(non_snake_case)]
use ac_library::Segtree;
use itertools::Itertools;
use proconio::{fastout, input};

#[fastout]
fn main() {
  input! {n:usize,m:usize,tws:[(u64,u64,u64);m]}

  let mut somen = vec![0; n];
  let mut st = Segtree::<ac_library::Min<u64>>::from(vec![0; n]);
  for (t, w, s) in tws {
    let p = st.max_right(0, |&m| m > t);
    if p < n {
      somen[p] += w;
      st.set(p, t + s);
    }
  }
  println!("{}", somen.iter().join("\n"));
}
