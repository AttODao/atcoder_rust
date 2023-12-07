use ac_library::{lcp_array, suffix_array};
use proconio::{fastout, input};

#[fastout]
fn main() {
  input! {s:String}

  println!(
    "{}",
    s.len() * (s.len() + 1) / 2 - lcp_array(&s, &suffix_array(&s)).into_iter().sum::<usize>()
  );
}
