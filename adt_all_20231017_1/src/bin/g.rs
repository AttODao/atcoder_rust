use std::process::exit;

use im_rc::HashSet;
use permutohedron::LexicalPermutation;
use proconio::{fastout, input};

fn dfs(i: usize, current: String, remain: isize, n: usize, s: &Vec<String>, t: &HashSet<String>) {
  if remain < 0 {
    return;
  }
  if i >= n {
    if t.contains(&current) || current.len() < 3 {
      return;
    } else {
      println!("{}", current);
      exit(0);
    }
  }
  if current.len() > 0 && current.chars().nth_back(0) != Some('_') {
    dfs(i, current + "_", remain, n, s, t);
  } else {
    dfs(i + 1, current.clone() + &s[i], remain, n, s, t);
    if current.len() > 0 {
      dfs(i, current + "_", remain - 1, n, s, t);
    }
  }
}

#[fastout]
fn main() {
  input! {n:usize,m:usize,mut s:[String;n],t:[String;m]}

  let t: HashSet<String> = t.into();
  s.sort();
  let remain: isize = (17 - s.iter().map(String::len).sum::<usize>() - n) as isize;
  loop {
    dfs(0, "".to_string(), remain, n, &s, &t);
    if !s.next_permutation() {
      break;
    }
  }
  println!("-1");
}
