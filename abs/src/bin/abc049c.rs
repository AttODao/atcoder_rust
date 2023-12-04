use proconio::input;

fn main() {
  input! {s:String}

  let n = s.len();
  let target = ["dream", "dreamer", "erase", "eraser"];
  let mut dp = [false; 100010];
  dp[0] = true;
  for i in 0..n {
    if dp[i] {
      for t in target.iter() {
        if s[i..].starts_with(t) {
          dp[i + t.len()] = true;
        }
      }
    }
  }
  if dp[n] {
    print!("YES");
  } else {
    print!("NO");
  }
}
