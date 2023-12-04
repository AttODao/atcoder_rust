use proconio::{fastout, input};

#[fastout]
fn main() {
  input! {n:usize,a:usize,b:usize}

  for i in 0..n * a {
    for j in 0..n * b {
      print!("{}", if i / a % 2 == j / b % 2 { "." } else { "#" });
    }
    print!("\n");
  }
}
