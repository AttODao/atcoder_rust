use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
  input! {n:usize,s:Chars}

  let mut ans: Vec<char> = Vec::new();
  let mut depth: Vec<isize> = vec![0; 1];
  let mut deletable = 0;
  for c in s {
    match c {
      '(' => {
        ans.push('(');
        depth.push(depth.last().unwrap() + 1);
        deletable += 1;
      }
      ')' => {
        if deletable > 0 {
          deletable -= 1;
          let d = *depth.last().unwrap();
          while *depth.last().unwrap() == d {
            ans.pop();
            depth.pop();
          }
        } else {
          ans.push(')');
          depth.push(depth.last().unwrap() - 1);
        }
      }
      _ => {
        ans.push(c);
        depth.push(*depth.last().unwrap());
      }
    }
  }
  println!("{}", ans.iter().collect::<String>());
}
