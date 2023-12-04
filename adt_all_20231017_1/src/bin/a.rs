use proconio::{fastout, input};

#[fastout]
fn main() {
  input! {s:String}
  if s == "Hello,World!" {
    println!("AC");
  } else {
    println!("WA");
  }
}
