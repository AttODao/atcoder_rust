#![allow(non_snake_case)]
use ac_library::Dsu;
use proconio::{fastout, input, marker::Usize1};

#[fastout]
fn main() {
  input! {n:usize,m:usize,ab:[(Usize1,Usize1);m]}
  let mut uf = Dsu::new(n);
  for (a, b) in ab {
    uf.merge(a, b);
  }
  println!(
    "{}",
    uf.groups()
      .into_iter()
      .map(|g| {
        let s = g.len();
        s * (s - 1) / 2
      })
      .sum::<usize>()
      - m
  );
}
