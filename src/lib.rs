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
#[derive(Debug)]
pub struct StrSplit<'a> {
  remainder: Option<&'a str>,
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
      remainder: Some(haystack), 
      delimiter
    }
  }
}

impl<'a> Iterator for StrSplit<'a> {
  // Item will live as long as the remainder, we have to specify that when defining it
  type Item = &'a str;

  // This is the only thing we need for an iterator
  fn next(&mut self) -> Option<Self::Item> {
    
    // There's something inside the remainder
    // ref mut keyword creates a &mut out of self.remainder
    // If we haven't used it, we'd have a new variable named remainder and couldn't
    // modify the original one
    // ? operator returns None if Option is None, otherwise returns what's inside
    // Without `as_mut` it would return a copy of the remainder 
    let remainder = self.remainder.as_mut()?;
    
    if let Some(next_delim) = remainder.find(self.delimiter) {
      let until_delimiter = &remainder[..next_delim];
      *remainder = &remainder[(next_delim + self.delimiter.len())..];
      Some(until_delimiter)
    }
    else {
      self.remainder.take()
    }
  }
}

#[test]
fn it_works() {
  let haystack = "a b c d e";
  let letters = StrSplit::new(haystack, " ");   

  let letters: Vec<_> = letters.collect();
  assert_eq!(letters, vec!["a", "b", "c", "d", "e"]);
}

fn main() {
  let haystack = "a b c d e";
  let letters = StrSplit::new(haystack, " ");   

  let letters: Vec<_> = letters.collect();
  println!("{:?}", letters);
}


#[test]
fn tail() {
  let haystack = "a b c d ";
  let letters = StrSplit::new(haystack, " ");   

  // If iterators are of same type, they can be compared. Lengths and each elements
  // is tested against each other
  assert!(letters.eq(vec!["a", "b", "c", "d", ""].into_iter()));
}
