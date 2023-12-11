#![allow(non_snake_case)]
use proconio::{fastout, input};

#[fastout]
fn main() {
  input! {n:usize,m:u64,l:[u64;n]}

  let lines = |w: u64| {
    let mut ret = 1;
    let mut r = 0;
    for &l in &l {
      r += l;
      if r > w {
        if l > w {
          return u64::MAX;
        }
        ret += 1;
        r = l;
      }
      r += 1;
    }
    ret
  };
  let (mut ng, mut ok) = (0, l.iter().sum::<u64>() + l.len() as u64);
  while ok - ng > 1 {
    let mid = (ok + ng) / 2;
    if lines(mid) <= m {
      ok = mid;
    } else {
      ng = mid;
    }
  }
  println!("{}", ok);
}
