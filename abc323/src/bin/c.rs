use std::cmp::Reverse;

use itertools::{izip, Itertools};
use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
  input! {n:usize,m:usize,a:[i32;m],s:[Chars;n]}

  let mut score = s
    .iter()
    .enumerate()
    .map(|(i, s)| {
      izip!(a.iter(), s.iter())
        .filter_map(|(&a, &s)| if s == 'o' { Some(a) } else { None })
        .sum::<i32>()
        + i as i32
    })
    .collect_vec();
  let max = *score.iter().max().unwrap();
  for (score, s) in izip!(score.iter_mut(), s.iter()) {
    let mut notyet = izip!(a.iter(), s.iter())
      .filter_map(|(&a, &s)| if s == 'x' { Some(a) } else { None })
      .sorted_by_key(|&a| Reverse(a));
    let mut ans = 0;
    while *score < max {
      *score += notyet.next().unwrap();
      ans += 1;
    }
    println!("{}", ans);
  }
}
