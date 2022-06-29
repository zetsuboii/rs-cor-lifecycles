// Using deny instead of warn may break the application in future Rust versions
#![warn(missing_debug_implementations, rust_2018_idioms, missing_docs)]

pub struct StrSplit {}

impl StrSplit {
  // haystack is what we are splitting
  // delimiter is by what we are splitting
  pub fn new (haystack: &str, delimiter: &str) -> Self {}
}

impl Iterator for StrSplit {
  type Item = &str;

  // This is the only thing we need for an iterator
  fn next(&mut self) -> Option<Self::Item>  {}
}

#[test]
fn it_works() {
  let haystack = "a b c d e";
  let letters = StrSplit::new(haystack, " ");   

  // If iterators are of same type, they can be compared. Lengths and each elements
  // is tested against each other
  assert_eq!(letters, vec!["a", "b", "c", "d", "e"].into_iter());
}


