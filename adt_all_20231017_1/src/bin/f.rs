use im_rc::HashSet;
use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
  input! {n:usize,s:Chars}

  let mut set: HashSet<(i32, i32)> = HashSet::new();
  let (mut x, mut y) = (0, 0);
  set.insert((0, 0));
  for c in s {
    match c {
      'R' => x += 1,
      'L' => x -= 1,
      'U' => y += 1,
      'D' => y -= 1,
      _ => (),
    }
    if set.contains(&(x, y)) {
      println!("Yes");
      return;
    }
    set.insert((x, y));
  }
  println!("No");
}
