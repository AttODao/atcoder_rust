#![allow(non_snake_case)]
use itertools::iproduct;
use proconio::{fastout, input};

fn f2(x1: i32, y1: i32, z1: i32, x2: i32, y2: i32, z2: i32) -> i32 {
  0.max(7 + x1.min(x2) - x1.max(x2))
    * 0.max(7 + y1.min(y2) - y1.max(y2))
    * 0.max(7 + z1.min(z2) - z1.max(z2))
}
fn f3(x1: i32, y1: i32, z1: i32, x2: i32, y2: i32, z2: i32, x3: i32, y3: i32, z3: i32) -> i32 {
  0.max(7 + x1.min(x2.min(x3)) - x1.max(x2.max(x3)))
    * 0.max(7 + y1.min(y2.min(y3)) - y1.max(y2.max(y3)))
    * 0.max(7 + z1.min(z2.min(z3)) - z1.max(z2.max(z3)))
}

#[fastout]
fn main() {
  input! {v1:i32,v2:i32,v3:i32}
  for (x1, y1, z1, x2, y2, z2) in iproduct!(-7..=7, -7..=7, -7..=7, -7..=7, -7..=7, -7..=7) {
    let u3 = f3(0, 0, 0, x1, y1, z1, x2, y2, z2);
    let u2 =
      f2(0, 0, 0, x1, y1, z1) + f2(0, 0, 0, x2, y2, z2) + f2(x1, y1, z1, x2, y2, z2) - 3 * u3;
    let u1 = 7 * 7 * 7 * 3 - 2 * u2 - 3 * u3;
    if (u1, u2, u3) == (v1, v2, v3) {
      println!("Yes\n0 0 0 {} {} {} {} {} {}", x1, y1, z1, x2, y2, z2);
      return;
    }
  }
  println!("No");
}
