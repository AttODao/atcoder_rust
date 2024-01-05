#![allow(non_snake_case)]
use ac_library::{LazySegtree, MapMonoid, ModInt998244353, Monoid};
use proconio::{fastout, input};

type Mint = ModInt998244353;
struct M;
impl Monoid for M {
  type S = (Mint, Mint);
  fn identity() -> Self::S {
    (0.into(), 0.into())
  }
  fn binary_operation(a: &Self::S, b: &Self::S) -> Self::S {
    (a.0 + b.0, a.1 + b.1)
  }
}
struct MM;
impl MapMonoid for MM {
  type M = M;
  type F = (Mint, Mint);
  fn identity_map() -> Self::F {
    (1.into(), 0.into())
  }
  fn mapping(
    (a, b): &Self::F,
    x: &<Self::M as ac_library::Monoid>::S,
  ) -> <Self::M as ac_library::Monoid>::S {
    (a * x.0 + b * x.1, x.1)
  }
  fn composition((fa, fb): &Self::F, (ga, gb): &Self::F) -> Self::F {
    (fa * ga, fa * gb + fb)
  }
}
#[fastout]
fn main() {
  input! {n:usize,m:usize}
  let mut dp = LazySegtree::<MM>::from(vec![(0.into(), 1.into()); m]);
  dp.set(0, (1.into(), 1.into()));
  for _ in 1..n {
    dp.apply_range(0..m, (-Mint::new(1), dp.all_prod().0));
  }
  println!("{}", dp.prod(1..m).0 * m);
}
