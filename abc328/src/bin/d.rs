use proconio::{fastout, input, marker::Chars};

const TARGET: [char; 3] = ['A', 'B', 'C'];

#[fastout]
fn main() {
  input! {s:Chars}

  let mut ans = vec!['.', '.'];
  for c in s {
    ans.push(c);
    let len = ans.len();
    if ans[(len - 3)..] == TARGET {
      ans.truncate(len - 3);
    }
  }
  println!("{}", ans.iter().skip(2).collect::<String>());
}
