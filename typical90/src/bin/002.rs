use itertools::Itertools;
use proconio::{fastout, input};

#[fastout]
fn main() {
  input! {n:i32}

  if n % 2 == 1 {
    return;
  }
  for a in &mut (0..n).combinations(n as usize / 2) {
    let mut depth = -a[0];
    for (a, b) in a.iter().tuple_windows() {
      if depth < 0 {
        break;
      }
      depth += 2 + a - b;
    }
    if depth >= 0 {
      let mut index = 0;
      println!(
        "{}",
        (0..n)
          .map(|i| if index < n as usize / 2 && i == a[index] {
            index += 1;
            '('
          } else {
            ')'
          })
          .collect::<String>()
      )
    }
  }
}
