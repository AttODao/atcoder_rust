use proconio::input;

fn main() {
  input! {n:usize, plan:[(i32,i32,i32);n]}

  let (mut pt, mut px, mut py) = (0, 0, 0);
  for (t, x, y) in plan {
    let dist = (x - px).abs() + (y - py).abs();
    if t - pt < dist || (dist - t + pt) & 1 == 1 {
      print!("No");
      return;
    }
    pt = t;
    px = x;
    py = y;
  }
  print!("Yes");
}
