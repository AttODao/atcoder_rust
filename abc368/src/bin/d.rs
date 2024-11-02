#![allow(non_snake_case)]
use im_rc::HashSet;
use proconio::{fastout, input, marker::Usize1};

#[fastout]
fn main() {
  input! {n:usize,k:usize,ab:[(Usize1,Usize1);n-1],v:[Usize1;k]}
  let mut edges = vec![HashSet::new(); n];
  let v = v.into_iter().collect::<HashSet<usize>>();
  for (a, b) in ab {
    edges[a].insert(b);
    edges[b].insert(a);
  }
  let mut ans = n;
  for i in 0..n {
    let mut j = i;
    while edges[j].len() == 1 && !v.contains(&j) {
      let &nj = edges[j].iter().nth(0).unwrap();
      edges[j].remove(&nj);
      edges[nj].remove(&j);
      ans -= 1;
      j = nj;
    }
  }
  println!("{ans}");
}
