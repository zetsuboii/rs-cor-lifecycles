// Using deny instead of warn may break the application in future Rust versions
#![warn(missing_debug_implementations, rust_2018_idioms, missing_docs)]

// 'a here is a generic way to say "This variable lives for this long".
// Giving both remainder and delimiter the same lifetime specifier, we implicitly
// say "Both remainder and delimiter lives for the same amount of lifetime which we
// call 'a "

// Giving a generic 'a as the lifetime, we can have remainders and delimiters that
// live longer than the StrSplit itself. Because it makes quite sense to pass a string
// to StrSplit::new and use the remainder of it after StrSplit has been dropped.

// When we are defining a generic type we can use <> right after the name we're defining
// be it a method, a struct or an enum
pub struct StrSplit<'a> {
  remainder: &'a str,
  delimiter: &'a str
}

// Implementations are for concerete types so `impl StrSplit<'a>` mean there's a literal
// type named `StrSplit<'a>`. In order to have generic lifetimes for impl's we have to
// define the lifetime after `impl` keyword
impl<'a> StrSplit<'a> {
  // 'haystack' is what we are splitting
  // 'delimiter' is by what we are splitting

  // As we're using generic lifetimes in the StrSplit, remainder and delimiter have
  // the lifetime of haystack and delimiter respectively

  // If it comes as confusing, think about the underlying string and the desired effect
  // of StrSplit on it, should it deallocate the string as it is dropped, or is the 
  // lifetime of the string might be longer than the StrSplit?
  pub fn new (haystack: &'a str, delimiter: &'a str) -> Self {
    // No need to use StrSplit for type of self
    Self {
      remainder: haystack, 
      delimiter
    }
  }
}

impl<'a> Iterator for StrSplit<'a> {
  // Item will live as long as the remainder, we have to specify that when defining it
  type Item = &'a str;

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
    } else if self.remainder.is_empty() {
      // TODO
      None 
    } else {

      // Return the last partition
      let rest = self.remainder;
      // Now the remainder is empty
      self.remainder = &[];
      Some(rest)
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


