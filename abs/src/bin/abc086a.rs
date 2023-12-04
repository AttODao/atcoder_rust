use proconio::input;

fn main() {
  input! {a:i32,b:i32,}
  if a & b & 1 == 1 {
    print!("Odd");
  } else {
    print!("Even");
  }
}
