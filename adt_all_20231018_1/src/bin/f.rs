use proconio::{fastout, input};

#[fastout]
fn main() {
  input! {mut k:usize}

  let mut ans: Vec<char> = Vec::new();
  while k > 0 {
    ans.push(if k & 1 == 1 { '2' } else { '0' });
    k >>= 1;
  }
  println!("{}", ans.iter().rev().collect::<String>())
}
