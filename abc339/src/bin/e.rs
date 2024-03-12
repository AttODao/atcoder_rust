#![allow(non_snake_case)]
use ac_library::{Max, Segtree};
use proconio::{fastout, input, marker::Usize1};

#[fastout]
fn main() {
  input! {n:usize,d:usize,a:[Usize1;n]}
  let mut st = Segtree::<Max<usize>>::new(500000);
  for a in a {
    st.set(a, st.prod(a.saturating_sub(d)..(a + d + 1).min(500000)) + 1);
  }
  println!("{}", st.all_prod());
}
