use proconio::input;

fn main() {
  input! {n:u32,mut a:[i32;n]}

  a.sort();
  let mut ans = 0;
  let mut sign = 1;
  for a in a {
    ans += sign * a;
    sign *= -1;
  }

  print!("{}", i32::abs(ans));
}
