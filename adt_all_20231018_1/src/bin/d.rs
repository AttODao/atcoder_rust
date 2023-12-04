use proconio::{fastout, input};

#[fastout]
fn main() {
  input! {n:usize,s:[String;n]}

  for i in 0..n {
    for j in 0..n {
      if i != j {
        let t = s[i].clone() + &s[j];
        if t == t.chars().rev().collect::<String>() {
          println!("Yes");
          return;
        }
      }
    }
  }
  println!("No");
}
