use std::iter::FromIterator;

use im_rc::HashSet;
use itertools::Itertools;
use proconio::{fastout, input, marker::Usize1};

#[fastout]
fn main() {
  input! {n:usize,a:[[i32;n];n],m:usize,xy:[(Usize1,Usize1);m]}

  let xy = HashSet::<(usize, usize)>::from_iter(xy);
  let mut ans = i32::MAX;
  for perm in (0..n).permutations(n) {
    let mut flag = true;
    for (&p, &q) in perm.iter().tuple_windows() {
      if xy.contains(&(p, q)) || xy.contains(&(q, p)) {
        flag = false;
        break;
      }
    }
    if flag {
      let mut time = 0;
      for (i, &p) in perm.iter().enumerate() {
        time += a[p][i];
      }
      ans = ans.min(time);
    }
  }
  println!("{}", if ans < i32::MAX { ans } else { -1 });
}
