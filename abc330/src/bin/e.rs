use std::{collections::BTreeSet, iter::FromIterator};

use proconio::{fastout, input, marker::Usize1};

#[fastout]
fn main() {
  input! {n:usize,q:usize,mut a:[usize;n],ix:[(Usize1,usize);q]}

  let mut cnt = vec![0; n + 1];
  let mut set = BTreeSet::from_iter(0..=n);
  for &a in a.iter() {
    if a <= n {
      cnt[a] += 1;
      set.remove(&a);
    }
  }
  for (i, x) in ix {
    if a[i] <= n {
      cnt[a[i]] -= 1;
      if cnt[a[i]] == 0 {
        set.insert(a[i]);
      }
    }
    if x <= n {
      cnt[x] += 1;
      set.remove(&x);
    }
    a[i] = x;
    println!("{}", set.iter().next().unwrap());
  }
}
