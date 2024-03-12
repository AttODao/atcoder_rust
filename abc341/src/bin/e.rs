#![allow(non_snake_case)]
use ac_library::{Min, Segtree};
use proconio::{
  fastout, input,
  marker::{Chars, Usize1},
};

#[fastout]
fn main() {
  input! {n:usize,q:usize,s:Chars,tlr:[(u8,Usize1,Usize1);q]}
  let mut st = Segtree::<Min<i8>>::new(n + 1);
  for i in 0..n - 1 {
    st.set(i + 1, if s[i] == s[i + 1] { 0 } else { 1 });
  }
  for (t, l, r) in tlr {
    if t == 1 {
      st.set(l, 1 - st.get(l));
      st.set(r + 1, 1 - st.get(r + 1));
    } else {
      println!(
        "{}",
        if l == r || st.prod(l + 1..r + 1) == 1 {
          "Yes"
        } else {
          "No"
        }
      )
    }
  }
}
