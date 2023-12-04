use proconio::{fastout, input};

#[fastout]
fn main() {
  input! {mut s:[usize;8]}

  s.push(10000);
  for i in 0..8 {
    if s[i] > s[i + 1] || s[i] < 100 || 675 < s[i] || s[i] % 25 != 0 {
      println!("No");
      return;
    }
  }
  println!("Yes");
}
