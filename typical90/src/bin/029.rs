use std::collections::BTreeMap;

use ac_library::{LazySegtree, MapMonoid};
use proconio::{fastout, input};

struct Map;
impl MapMonoid for Map {
  type M = ac_library::Max<u32>;
  type F = u32;
  fn identity_map() -> Self::F {
    0
  }
  fn mapping(
    f: &Self::F,
    x: &<Self::M as ac_library::Monoid>::S,
  ) -> <Self::M as ac_library::Monoid>::S {
    *x.max(f)
  }
  fn composition(f: &Self::F, g: &Self::F) -> Self::F {
    *f.max(g)
  }
}

#[fastout]
fn main() {
  input! {_:u32,n:usize,lr:[(u32,u32);n]}

  let mut indexes = BTreeMap::<u32, Vec<(usize, u8)>>::new();
  for (i, (l, r)) in lr.into_iter().enumerate() {
    indexes.entry(l).or_default().push((i, 0));
    indexes.entry(r).or_default().push((i, 1));
  }
  let mut bricks = vec![(0, 0); n];
  for (compressed, indexes) in indexes.values().enumerate() {
    for &(i, l_or_r) in indexes {
      if l_or_r == 0 {
        bricks[i].0 = compressed;
      } else {
        bricks[i].1 = compressed;
      }
    }
  }
  let mut st = LazySegtree::<Map>::new(indexes.len());
  for &(l, r) in &bricks {
    let max = st.prod(l..=r);
    st.apply_range(l..=r, max + 1);
    println!("{}", max + 1);
  }
}
