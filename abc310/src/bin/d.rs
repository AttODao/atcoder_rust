#![allow(non_snake_case)]
use im_rc::HashSet;
use proconio::{fastout, input, marker::Usize1};

#[fastout]
fn main() {
  input! {n:usize,t:usize,ab:[(Usize1,Usize1)]}
  let mut ok = vec![vec![true; n]; n];
  for (a, b) in ab {
    ok[a][b] = false;
    ok[b][a] = false;
  }
  let mut ans = 0;
  let mut dfs: Vec<(usize, Vec<Vec<usize>>)> = vec![];
  dfs.push((0, vec![]));
  while let Some((i, mut teams)) = dfs.pop() {
    if i < n {
      for t in 0..teams.len() {
        if teams[t].iter().all(|&j| ok[i][j]) {
          let mut teams = teams.clone();
          teams[t].push(i);
          dfs.push((i + 1, teams));
        }
      }
      if teams.len() < t {
        teams.push(vec![i]);
        dfs.push((i + 1, teams));
      }
    } else if teams.len() == t {
      ans += 1;
    }
  }
  println!("{}", ans);
}
