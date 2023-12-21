#![allow(non_snake_case)]
use itertools::{izip, Itertools};
use proconio::{
  fastout, input,
  marker::{Chars, Usize1},
};

#[fastout]
fn main() {
  input! {n:usize,m:usize,s:Chars,c:[Usize1;n]}

  let mut table = vec![vec![]; m];
  let mut mark = vec![(0, 0); n];
  for (i, (s, c)) in izip!(s, c).enumerate() {
    mark[i] = (c, table[c].len());
    table[c].push(s);
  }
  table.iter_mut().for_each(|t| t.rotate_right(1));
  println!("{}", mark.into_iter().map(|(c, i)| table[c][i]).join(""));
}
