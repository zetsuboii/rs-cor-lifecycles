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
pub struct StrSplit<'hs, D> {
  remainder: Option<&'hs str>,
  delimiter: D,
}

// Implementations are for concerete types so `impl StrSplit<'a>` mean there's a literal
// type named `StrSplit<'a>`. In order to have generic lifetimes for impl's we have to
// define the lifetime after `impl` keyword
impl<'hs, D> StrSplit<'hs, D> {
  // 'haystack' is what we are splitting
  // 'delimiter' is by what we are splitting

  // As we're using generic lifetimes in the StrSplit, remainder and delimiter have
  // the lifetime of haystack and delimiter respectively

  // If it comes as confusing, think about the underlying string and the desired effect
  // of StrSplit on it, should it deallocate the string as it is dropped, or is the
  // lifetime of the string might be longer than the StrSplit?
  pub fn new(haystack: &'hs str, delimiter: D) -> Self {
    // No need to use StrSplit for type of self
    Self {
      remainder: Some(haystack),
      delimiter,
    }
  }
}

pub trait Delimiter {
  /// Finds self in the string s and returns where it starts and where it ends
  fn find_next(&self, s: &str) -> Option<(usize, usize)>;
}

impl Delimiter for char {
  fn find_next(&self, s: &str) -> Option<(usize, usize)> {
    s.char_indices()
      .position(|(i,c)| c == *self)
      .map(|i| (i, i+1))
  }
}

impl Delimiter for &str {
  fn find_next(&self, s: &str) -> Option<(usize, usize)> {
    s.find(*self).map(|start| (start, start+self.len()))
  }
}

// We can use '_ as we won't need delimiter's lifetime in the code, and it can be 
// inferred
impl<'hs, D> Iterator for StrSplit<'hs, D>
where D: Delimiter
{
  // Item will live as long as the remainder, we have to specify that when defining it
  type Item = &'hs str;

  // This is the only thing we need for an iterator
  fn next(&mut self) -> Option<Self::Item> {
    // There's something inside the remainder
    // ref mut keyword creates a &mut out of self.remainder
    // If we haven't used it, we'd have a new variable named remainder and couldn't
    // modify the original one
    // ? operator returns None if Option is None, otherwise returns what's inside
    // Without `as_mut` it would return a copy of the remainder
    let remainder = self.remainder.as_mut()?;

    if let Some((delim_start, delim_end)) = self.delimiter.find_next(&remainder) {
      let until_delimiter = &remainder[..delim_start];
      *remainder = &remainder[delim_end..];
      Some(until_delimiter)
    } else {
      self.remainder.take()
    }
  }
}

fn until_char(s: &str, c: char) -> &'_ str {
  StrSplit::new(s, c)
    .next()
    .expect("StrSplit always returns at least one result")
}

#[test]
fn until_char_works() {
  let haystack = "abcde";
  assert_eq!(until_char(haystack, 'c'), "ab");
}

#[test]
fn it_works() {
  let haystack = "a b c d e";
  let letters = StrSplit::new(haystack, " ");

  let letters: Vec<_> = letters.collect();
  assert_eq!(letters, vec!["a", "b", "c", "d", "e"]);
}

#[test]
fn empty() {
  let haystack = "a b c d e";
  let letters = StrSplit::new(haystack, ",");

  assert_eq!(letters.collect::<Vec<_>>(), vec![haystack]);
}

#[test]
fn tail() {
  let haystack = "a b c d ";
  let letters = StrSplit::new(haystack, " ");

  // If iterators are of same type, they can be compared. Lengths and each elements
  // is tested against each other
  assert!(letters.eq(vec!["a", "b", "c", "d", ""].into_iter()));
}
