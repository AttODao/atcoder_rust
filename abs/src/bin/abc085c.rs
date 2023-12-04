use std::cmp::Ordering;

use proconio::input;

fn main() {
  input! {n:u32,y:u32}

  for i in 0..=n {
    if 10000 * i + 1000 * (n - i) > y {
      break;
    }
    for j in 0..=n - i {
      match y.cmp(&(10000 * i + 5000 * j + 1000 * (n - i - j))) {
        Ordering::Less => break,
        Ordering::Equal => {
          print!("{} {} {}", i, j, n - i - j);
          return;
        }
        Ordering::Greater => (),
      }
    }
  }
  print!("-1 -1 -1");
}
