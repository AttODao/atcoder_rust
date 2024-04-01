#![allow(non_snake_case)]
use itertools::Itertools;
use proconio::{fastout, input, marker::Usize1};

#[fastout]
fn main() {
  input! {h:usize,w:usize,tax:[(u8,Usize1,usize)]}
  let mut r = vec![true; h];
  let mut c = vec![true; w];
  let (mut hr, mut hc) = (h, w);
  let mut cnt = vec![0; 200010];
  for (t, a, x) in tax.into_iter().rev() {
    if t == 1 {
      if r[a] {
        r[a] = false;
        cnt[x] += hc;
        hr -= 1;
      }
    } else {
      if c[a] {
        c[a] = false;
        cnt[x] += hr;
        hc -= 1;
      }
    }
  }
  cnt[0] += hr * hc;
  let mut ans = vec![];
  for x in 0..200010 {
    if cnt[x] > 0 {
      ans.push((x, cnt[x]));
    }
  }
  println!(
    "{}\n{}",
    ans.len(),
    ans
      .into_iter()
      .map(|(x, cnt)| format!("{x} {cnt}"))
      .join("\n")
  );
}
