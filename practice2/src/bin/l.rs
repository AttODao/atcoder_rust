use ac_library::{LazySegtree, MapMonoid, Monoid};
use itertools::Itertools;
use proconio::{fastout, input, marker::Usize1};

struct InvNum;
impl Monoid for InvNum {
  type S = (usize, usize, usize);
  fn identity() -> Self::S {
    (0, 0, 0)
  }
  fn binary_operation(a: &Self::S, b: &Self::S) -> Self::S {
    (a.0 + b.0 + a.2 * b.1, a.1 + b.1, a.2 + b.2)
  }
}
struct InvMap;
impl MapMonoid for InvMap {
  type M = InvNum;
  type F = bool;
  fn identity_map() -> Self::F {
    false
  }
  fn mapping(f: &Self::F, x: &<Self::M as Monoid>::S) -> <Self::M as Monoid>::S {
    if *f {
      (x.1 * x.2 - x.0, x.2, x.1)
    } else {
      *x
    }
  }
  fn composition(f: &Self::F, g: &Self::F) -> Self::F {
    f ^ g
  }
}

#[fastout]
fn main() {
  input! {n:usize,q:usize,a:[usize;n]}

  let mut lst = LazySegtree::<InvMap>::from(a.into_iter().map(|a| (0, 1 - a, a)).collect_vec());
  for _ in 0..q {
    input! {t:u8,l:Usize1,r:Usize1}
    match t {
      1 => {
        lst.apply_range(l..=r, true);
      }
      2 => {
        println!("{}", lst.prod(l..=r).0);
      }
      _ => (),
    }
  }
}
