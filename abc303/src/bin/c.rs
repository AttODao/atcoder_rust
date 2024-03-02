#![allow(non_snake_case)]
use im_rc::HashSet;
use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
  input! {_:usize,m:usize,mut h:usize,k:usize,s:Chars,xy:[(i32,i32);m]}
  let mut items = xy.into_iter().collect::<HashSet<(i32, i32)>>();
  let (mut x, mut y) = (0, 0);
  for c in s {
    if h == 0 {
      println!("No");
      return;
    }
    match c {
      'R' => x += 1,
      'L' => x -= 1,
      'U' => y += 1,
      'D' => y -= 1,
      _ => (),
    }
    h -= 1;
    if items.contains(&(x, y)) && h < k {
      h = k;
      items.remove(&(x, y));
    }
  }
  println!("Yes");
}
