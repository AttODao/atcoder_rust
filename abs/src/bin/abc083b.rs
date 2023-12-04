use proconio::input;

fn sod(mut x: u32) -> u32 {
  let mut ret = 0;
  while x > 0 {
    ret += x % 10;
    x /= 10;
  }
  ret
}

fn main() {
  input! {n:u32,a:u32,b:u32}

  let mut ans: u32 = 0;
  for i in 1..=n {
    let sod = sod(i);
    if a <= sod && sod <= b {
      ans += i;
    }
  }

  print!("{}", ans);
}
