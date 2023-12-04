use proconio::input;
use std::cmp;

fn main() {
  input!(a:i32,b:i32,c:i32,d:i32,);
  print!("{}", cmp::max(a - d, b - c));
}
