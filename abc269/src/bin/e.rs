use proconio::input_interactive;

fn main() {
  input_interactive!(n:usize);

  let (mut ok1, mut ng1) = (1, n + 1);
  while ng1 - ok1 > 1 {
    let mid = (ok1 + ng1) / 2;
    println!("? {} {} 1 {}", ok1, mid - 1, n);
    input_interactive!(t:usize);
    if t == mid - ok1 {
      ok1 = mid;
    } else {
      ng1 = mid;
    }
  }
  let (mut ok2, mut ng2) = (1, n + 1);
  while ng2 - ok2 > 1 {
    let mid = (ok2 + ng2) / 2;
    println!("? 1 {} {} {}", n, ok2, mid - 1);
    input_interactive!(t:usize);
    if t == mid - ok2 {
      ok2 = mid;
    } else {
      ng2 = mid;
    }
  }
  println!("! {} {}", ok1, ok2);
}
