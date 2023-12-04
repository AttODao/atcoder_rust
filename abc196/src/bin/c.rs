use proconio::input;

fn main() {
  input! {n:i64}

  let mut ans = 0;
  for i in 1..1000000i64 {
    let mut dig = 1;
    while dig <= i {
      dig *= 10;
    }
    let num = i * dig + i;
    if num <= n {
      ans += 1;
    }
  }
  println!("{}", ans);
}
