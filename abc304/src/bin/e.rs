#![allow(non_snake_case)]
use ac_library::Dsu;
use im_rc::HashSet;
use itertools::Itertools;
use proconio::{fastout, input, marker::Usize1};

#[fastout]
fn main() {
  input! {n:usize,uv:[(Usize1,Usize1)],xy:[(Usize1,Usize1)],pq:[(Usize1,Usize1)]}
  let mut dsu = Dsu::new(n);
  for (u, v) in uv {
    dsu.merge(u, v);
  }
  let mut ng = HashSet::new();
  for (x, y) in xy {
    let (l1, l2) = (dsu.leader(x), dsu.leader(y));
    ng.insert((l1, l2));
    ng.insert((l2, l1));
  }
  println!(
    "{}",
    pq.into_iter()
      .map(|(p, q)| if ng.contains(&(dsu.leader(p), dsu.leader(q))) {
        "No"
      } else {
        "Yes"
      })
      .join("\n")
  );
}
