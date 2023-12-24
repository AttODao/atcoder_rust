#![allow(non_snake_case)]
use ac_library::{LazySegtree, MapMonoid, Monoid};
use itertools::Itertools;
use proconio::{fastout, input};

fn dist(p1: (f64, f64), p2: (f64, f64)) -> f64 {
  ((p1.0 - p2.0) * (p1.0 - p2.0) + (p1.1 - p2.1) * (p1.1 - p2.1)).sqrt()
}
struct M;
impl Monoid for M {
  type S = f64;

  fn identity() -> Self::S {
    f64::INFINITY
  }

  fn binary_operation(a: &Self::S, b: &Self::S) -> Self::S {
    a.min(*b)
  }
}
struct MM;
impl MapMonoid for MM {
  type M = M;
  type F = f64;
  fn identity_map() -> Self::F {
    0.0
  }
  fn mapping(
    f: &Self::F,
    x: &<Self::M as ac_library::Monoid>::S,
  ) -> <Self::M as ac_library::Monoid>::S {
    f + x
  }
  fn composition(f: &Self::F, g: &Self::F) -> Self::F {
    f + g
  }
}
#[fastout]
fn main() {
  input! {n:usize,k:usize,ps:(f64,f64),xy:[(f64,f64);n]}
  let mut dp = LazySegtree::<MM>::new(k);
  let mut dpone = 0;
  dp.set(k - 1, dist(ps, xy[0]));
  for (&p1, &p2) in xy.iter().tuple_windows() {
    dp.set(dpone, dp.all_prod() + dist(p1, ps) + dist(ps, p2));
    let dist = dist(p1, p2);
    dp.apply_range(0..dpone, dist);
    dp.apply_range(dpone + 1..k, dist);
    dpone = (dpone + 1) % k;
  }
  println!("{:.16}", dp.all_prod() + dist(*xy.last().unwrap(), ps));
}
