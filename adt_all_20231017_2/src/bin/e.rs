use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
  input! {h:usize,w:usize,s:[Chars;h],t:[Chars;h]}

  let (mut ts, mut tt) = (vec!["".to_string(); w], vec!["".to_string(); w]);
  for i in 0..h {
    for j in 0..w {
      ts[j].push(s[i][j]);
      tt[j].push(t[i][j]);
    }
  }
  ts.sort();
  tt.sort();
  if ts == tt {
    println!("Yes");
  } else {
    println!("No");
  }
}
