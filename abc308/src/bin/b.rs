#![allow(non_snake_case)]
use im_rc::HashMap;
use itertools::izip;
use proconio::{fastout, input};

#[fastout]
fn main() {
  input! {n:usize,m:usize,c:[String;n],d:[String;m],p:[u32;m+1]}
  let prices = izip!(d, p.iter().skip(1)).collect::<HashMap<String, &u32>>();
  println!(
    "{}",
    c.into_iter()
      .map(|c| if let Some(&p) = prices.get(&c) {
        *p
      } else {
        p[0]
      })
      .sum::<u32>()
  );
}
