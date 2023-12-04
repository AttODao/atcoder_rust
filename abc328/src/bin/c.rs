use itertools::Itertools;
use proconio::{
  fastout, input,
  marker::{Chars, Usize1},
};

#[fastout]
fn main() {
  input! {n:usize,q:usize,s:Chars,lr:[(Usize1,Usize1);q]}

  let mut table = vec![0; n];
  for (i, (a, b)) in s.iter().tuple_windows().enumerate() {
    table[i + 1] = table[i];
    if a == b {
      table[i + 1] += 1;
    }
  }

  for (l, r) in lr {
    println!("{}", table[r] - table[l]);
  }
}
