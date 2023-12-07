use ac_library::{LazySegtree, MapMonoid, ModInt998244353, Monoid};
use itertools::Itertools;
use proconio::{fastout, input};

type Mint = ModInt998244353;

struct Sum;
impl Monoid for Sum {
  type S = (Mint, Mint);
  fn identity() -> Self::S {
    (0.into(), 0.into())
  }
  fn binary_operation(a: &Self::S, b: &Self::S) -> Self::S {
    (a.0 + b.0, a.1 + b.1)
  }
}
struct MulMap;
impl MapMonoid for MulMap {
  type M = Sum;
  type F = (Mint, Mint);
  fn identity_map() -> Self::F {
    (1.into(), 0.into())
  }
  fn mapping(f: &Self::F, x: &<Self::M as Monoid>::S) -> <Self::M as Monoid>::S {
    (f.0 * x.0 + f.1 * x.1, x.1)
  }
  fn composition(f: &Self::F, g: &Self::F) -> Self::F {
    (f.0 * g.0, f.0 * g.1 + f.1)
  }
}

#[fastout]
fn main() {
  input! {n:usize,q:usize,a:[u128;n]}

  let mut lst =
    LazySegtree::<MulMap>::from(a.into_iter().map(|a| (a.into(), 1.into())).collect_vec());
  for _ in 0..q {
    input! {t:usize}
    match t {
      0 => {
        input! {l:usize,r:usize,b:u128,c:u128}
        lst.apply_range(l..r, (b.into(), c.into()));
      }
      1 => {
        input! {l:usize,r:usize}
        println!("{}", lst.prod(l..r).0);
      }
      _ => (),
    }
  }
}
