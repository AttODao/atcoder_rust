#![allow(non_snake_case)]

use itertools::{iproduct, izip, Itertools};
use proconio::{fastout, input, marker::Chars};

const TARGET: i32 = 0b1111011110111101111;

#[fastout]
fn main() {
  input! {p:[[Chars;4];3]}

  let mut a = vec![vec![]; 3];
  for (a, p) in izip!(a.iter_mut(), p.into_iter()) {
    let (mut x1, mut x2, mut x3, mut x4) = (0, 0, 0, 0);
    for (i, j) in iproduct!(0..4, 0..4) {
      if p[i][j] == '#' {
        x1 += 1 << (i + 5 * j);
        x2 += 1 << ((3 - i) + 5 * (3 - j));
        x3 += 1 << (5 * i + (3 - j));
        x4 += 1 << (5 * (3 - i) + j);
      }
    }
    x1 /= x1 & -x1;
    x2 /= x2 & -x2;
    x3 /= x3 & -x3;
    x4 /= x4 & -x4;
    a.append(&mut (0..20).map(|i| x1 << i).collect_vec());
    a.append(&mut (0..20).map(|i| x2 << i).collect_vec());
    a.append(&mut (0..20).map(|i| x3 << i).collect_vec());
    a.append(&mut (0..20).map(|i| x4 << i).collect_vec());
  }

  for &x in &a[0] {
    for &y in &a[1] {
      for &z in &a[2] {
        if y & z == 0 && z & x == 0 && x & y == 0 && x | y | z == TARGET {
          println!("Yes");
          return;
        }
      }
    }
  }
  println!("No");
}
