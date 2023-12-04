use proconio::{fastout, input};

#[fastout]
fn main() {
  input! {n:usize,mut a:[i64;n],q:usize,mut b:[i64;q]}

  a.push(i64::MIN / 2);
  a.push(i64::MAX / 2);
  a.sort();
  for b in b {
    let p = a.partition_point(|&a| a < b);
    println!("{}", (a[p] - b).min(b - a[p - 1]));
  }
}
