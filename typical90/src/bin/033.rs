use proconio::{fastout, input};

#[fastout]
fn main() {
  input! {h:u32,w:u32}
  println!(
    "{}",
    if h.min(w) > 1 {
      ((h + 1) / 2) * ((w + 1) / 2)
    } else {
      h * w
    }
  );
}
