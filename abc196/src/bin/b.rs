use proconio::input;

fn main() {
  input! {a:String}
  print!("{}", a.split('.').next().unwrap());
}
