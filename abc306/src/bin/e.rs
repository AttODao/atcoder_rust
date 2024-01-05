#![allow(non_snake_case)]
use std::{collections::BTreeSet, mem};

use proconio::{fastout, input, marker::Usize1};

#[fastout]
fn main() {
  input! {n:usize,k:usize,xy:[(Usize1,u64)]}
  let mut set1 = (0..k).map(|i| (0, i)).collect::<BTreeSet<_>>();
  let mut set2 = (k..n).map(|i| (0, i)).collect::<BTreeSet<_>>();
  let mut sum = 0;
  let mut a = vec![0; n];
  for (x, y) in xy {
    if set1.contains(&(a[x], x)) {
      set1.remove(&(a[x], x));
      sum -= a[x];
      set1.insert((y, x));
      sum += y;
    } else {
      set2.remove(&(a[x], x));
      set2.insert((y, x));
    }
    a[x] = y;
    if let Some(mut p) = set1.pop_first() {
      sum -= p.0;
      if let Some(mut q) = set2.pop_last() {
        if p < q {
          mem::swap(&mut p, &mut q);
        }
        set2.insert(q);
      }
      set1.insert(p);
      sum += p.0;
    }
    println!("{}", sum);
  }
}
