// Using deny instead of warn may break the application in future Rust versions
#![warn(missing_debug_implementations, rust_2018_idioms, missing_docs)]

pub struct StrSplit {
  remainder: &str,
  delimiter: &str
}

impl StrSplit {
  // haystack is what we are splitting
  // delimiter is by what we are splitting
  pub fn new (haystack: &str, delimiter: &str) -> Self {
    // No need to use StrSplit for type of self
    Self { remainder: haystack, delimiter }
  }
}

impl Iterator for StrSplit {
  type Item = &str;

  // This is the only thing we need for an iterator
  fn next(&mut self) -> Option<Self::Item> {
    if let Some(next_delim) = self.remainder.find(self.delimiter) {
      // oh my, is this legal?
      // --re I thought this was a string and freaked out but next_delim is the byte
      // index so it's all cool :)
      let until_delimiter = &self.remainder[..next_delim];
      // next_delim is of type usize because &str.find returns the byte index of the
      // searched pattern 
      self.remainder = &self.remainder[(next_delim + self.delimiter.len())..];
      Some(until_delimiter)
    }
  }
}

#[test]
fn it_works() {
  let haystack = "a b c d e";
  let letters = StrSplit::new(haystack, " ");   

  // If iterators are of same type, they can be compared. Lengths and each elements
  // is tested against each other
  assert_eq!(letters, vec!["a", "b", "c", "d", "e"].into_iter());
}


