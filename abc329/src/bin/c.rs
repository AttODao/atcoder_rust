use itertools::Itertools;
use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
  input! {n:usize,s:Chars}

  let mut s = s.iter().map(|&c| c as usize - 'a' as usize).collect_vec();
  s.push(100);
  let mut cnt = [0; 26];
  let mut consec = 0;
  for (&a, &b) in s.iter().tuple_windows() {
    consec += 1;
    if a != b {
      cnt[a] = cnt[a].max(consec);
      consec = 0;
    }
  }
  println!("{}", cnt.iter().sum::<usize>());
}
