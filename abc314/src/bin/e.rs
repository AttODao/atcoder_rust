#![allow(non_snake_case)]
use itertools::{izip, Itertools};
use proconio::{fastout, input};

#[fastout]
fn main() {
  input! {n:usize,m:usize}
  let mut c = vec![];
  let mut s = vec![];
  for _ in 0..n {
    input! {cc:f64,ss:[usize]}
    let sss = ss
      .iter()
      .filter_map(|&ss| if ss > 0 { Some(ss) } else { None })
      .collect_vec();
    c.push(cc * ss.len() as f64 / sss.len() as f64);
    s.push(sss);
  }
  let mut dp = vec![0.; m + 1];
  for i in (0..m).rev() {
    dp[i] = izip!(c.iter(), s.iter())
      .map(|(c, s)| c + s.iter().map(|s| dp[m.min(i + s)]).sum::<f64>() / s.len() as f64)
      .min_by(|x, y| x.partial_cmp(y).unwrap())
      .unwrap();
  }
  println!("{:.16}", dp[0]);
}
