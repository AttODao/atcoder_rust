use std::f64::consts::PI;

use proconio::{fastout, input};

#[fastout]
fn main() {
  input! {t:f64,l:f64,x:f64,y:f64,q:usize,e:[f64;q]}

  let r = l / 2.0;
  for e in e {
    let theta = 2.0 * PI * e / t;
    let y1 = -r * theta.sin();
    let z1 = r - r * theta.cos();
    let d = (x * x + (y - y1) * (y - y1)).sqrt();
    println!("{}", (z1 / d).atan() * 180.0 / PI);
  }
}
