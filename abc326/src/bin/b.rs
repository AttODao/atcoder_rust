use proconio::{fastout, input};

fn judge(n: usize) -> bool {
  let a = n / 100;
  let b = n / 10 % 10;
  let c = n % 10;
  a * b == c
}

#[fastout]
fn main() {
  input! {n:usize}

  for i in n..920 {
    if judge(i) {
      println!("{}", i);
      return;
    }
  }
}
