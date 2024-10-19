#![allow(non_snake_case)]
use itertools::Itertools;
use proconio::{fastout, input};

#[fastout]
fn main() {
  input! {n:usize,x:u64,y:u64,a:[u64;n],b:[u64;n]}
  println!(
    "{}",
    a.iter()
      .sorted()
      .rev()
      .scan(0, |sum, a| {
        let mut ret = None;
        if *sum <= x {
          ret = Some(());
        }
        *sum += a;
        ret
      })
      .count()
      .min(
        b.iter()
          .sorted()
          .rev()
          .scan(0, |sum, b| {
            let mut ret = None;
            if *sum <= y {
              ret = Some(());
            }
            *sum += b;
            ret
          })
          .count()
      )
  )
}
