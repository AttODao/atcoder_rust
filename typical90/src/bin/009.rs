use std::f64::consts::PI;

use alga::general::Lattice;
use itertools::Itertools;
use proconio::{fastout, input};

#[fastout]
fn main() {
  input! {n:usize,xy:[(f64,f64);n]}

  let mut ans = 0.0;
  for &p in xy.iter() {
    let mut args = xy
      .iter()
      .flat_map(|&q| {
        if p != q {
          let arg = f64::atan2(q.0 - p.0, q.1 - p.1);
          vec![arg, arg + 2.0 * PI]
        } else {
          vec![]
        }
      })
      .collect_vec();
    args.sort_by(|a, b| a.partial_cmp(b).unwrap());
    for i in 0..(n - 1) {
      let p = args.partition_point(|&a| a < args[i] + PI);
      ans = *ans
        .partial_max(
          (args[p - 1] - args[i])
            .partial_max(&(2.0 * PI - args[p] + args[i]))
            .unwrap(),
        )
        .unwrap();
    }
  }
  println!("{}", ans / PI * 180.0);
}
