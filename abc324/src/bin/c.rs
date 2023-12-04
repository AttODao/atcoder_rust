use itertools::{izip, Itertools};
use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
  input! {n:usize,t:Chars,s:[Chars;n]}

  let ans = izip!(1..=n, s)
    .filter_map(|(i, s)| {
      let common = izip!(s.iter(), t.iter())
        .take_while(|(s, t)| s == t)
        .count()
        + izip!(s.iter().rev(), t.iter().rev())
          .take_while(|(s, t)| s == t)
          .count();
      let (maxlen, minlen) = (s.len().max(t.len()), s.len().min(t.len()));
      if maxlen <= minlen + 1 && maxlen <= common + 1 {
        Some(i)
      } else {
        None
      }
    })
    .collect_vec();
  println!("{}\n{}", ans.len(), ans.iter().join(" "));
}
