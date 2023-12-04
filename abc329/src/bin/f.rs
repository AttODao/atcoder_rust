use std::mem::take;

use im_rc::hashset;
use itertools::Itertools;
use proconio::{fastout, input, marker::Usize1};

#[fastout]
fn main() {
  input! {n:usize,q:usize,c:[Usize1;n],ab:[(Usize1,Usize1);q]}
  let mut table = c.iter().map(|c| hashset![c]).collect_vec();
  for (a, b) in ab {
    if table[a].len() > table[b].len() {
      table.swap(a, b);
    }
    let t = take(&mut table[a]);
    table[b].extend(t);
    println!("{}", table[b].len());
  }
}
