use ac_library::Dsu;
use itertools::Itertools;
use proconio::{fastout, input, marker::Usize1};

#[fastout]
fn main() {
  input! {n:usize,m:usize,k:i128,uvw:[(Usize1,Usize1,i128);m]}

  let mut ans = i128::MAX;
  for edges in uvw.iter().combinations_with_replacement(n - 1) {
    let mut uf = Dsu::new(n);
    let mut cost = 0;
    for &(u, v, w) in edges {
      uf.merge(u, v);
      cost = (cost + w) % k;
    }
    if uf.size(0) == n {
      ans = ans.min(cost);
    }
  }
  println!("{}", ans);
}
