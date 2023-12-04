use proconio::marker::Chars;
use proconio::{fastout, input};

fn is_ok(n: u128, x: &Vec<u128>, m: u128) -> bool {
  let mut num = 0;
  for &x in x {
    num = num * n + x;
    if num > m {
      return false;
    }
  }
  true
}

#[fastout]
fn main() {
  input! {x:Chars,m:u128}

  let x: Vec<u128> = x
    .iter()
    .map(|x| x.to_digit(10).unwrap() as u128)
    .collect::<Vec<u128>>();
  let d = *x.iter().max().unwrap();
  if x.len() == 1 {
    if d <= m {
      println!("1");
    } else {
      println!("0");
    }
    return;
  }
  let (mut ok, mut ng) = (d, m + 1);
  while ng - ok > 1 {
    let mid = (ok + ng) / 2;
    if is_ok(mid, &x, m) {
      ok = mid;
    } else {
      ng = mid;
    }
  }
  println!("{}", ok - d);
}
