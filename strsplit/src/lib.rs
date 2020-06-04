// Adding the following lints
// Setting the level to warn and not deny
// since these might change over time and
// we don't want that to break our code later.
// #![warn(missing_debug_implementations, rust_2018_idioms, missing_docs)]

#[derive(Debug)]
pub struct StrSplit<'haystack, D> {
    remainder: Option<&'haystack str>,
    delimiter: D,
}

impl<'haystack, D> StrSplit<'haystack, D> {
    // using self is good since if we rename the type
    // we don't need to change the code here
    // however this prevents local reasoning. looking at
    // this line we don't understand much untill we know
    // which type we are in
    pub fn new(haystack: &'haystack str, delimiter: D) -> Self {
        Self {
            remainder: Some(haystack),
            delimiter,
        }
    }
}

pub trait Delimiter {
    // Given a string return an option with two numbers
    // where it start and where it end
    fn find_next(&self, s: &str) -> Option<(usize, usize)>;
}

// Allows to write code like this
// let x: StrSplit;
// for part in x {
// do_stuff_with_x
// }
impl<'haystack, D> Iterator for StrSplit<'haystack, D> where D: Delimiter {
    type Item = &'haystack str;
    fn next(&mut self) -> Option<Self::Item> {
        // self.remainder if has anything is Some(T) and in remainder
        // we want the mutable reference to it
        // Some(&mut remainder) won't work, since it would try to match
        // Some(&mut T) and not Some(T)
        if let Some(ref mut remainder) = self.remainder {
            if let Some((delim_start, delim_end)) = self.delimiter.find_next(&remainder) {
                let until_delimiter = &remainder[..delim_start];
                // remainder here is of type &mut 'a str
                // RHS is 'a str, that is why we are dereferencing the
                // remainder on LHS, basically we are saying put the string
                // into the place where remainder is pointing
                *remainder = &remainder[delim_end..];
                Some(until_delimiter)
            } else {
                // take is a function implemented on Options
                // it takes mutable reference to the Option and
                // and gives back Option
                // If the Option is None it returns None
                // else it sets the Option to None and returns
                // the Some that was in it
                self.remainder.take()
            }
        } else {
            None
        }
    }
}

impl Delimiter for &str {
    fn find_next(&self, s: &str) -> Option<(usize, usize)> {
        s.find(self).map(|start| {
            (start, start + self.len())
        })
    }
}

impl Delimiter for char {
    fn find_next(&self, s: &str) -> Option<(usize, usize)> {
        s.char_indices().find(|(_, c)| {
            c == self
        }).map(|(start, _)| {
            (start, start + self.len_utf8())
        })
    }
}

// Lifetime is inferred
pub fn until_char(s: &str, c: char) -> &'_ str {
    StrSplit::new(s, c)
        .next()
        .expect("StrSplit always gives atleast one result")
}

#[test]
fn until_char_test() {
    assert_eq!(until_char("hello world", 'o'), "hell");
}

#[test]
fn it_works() {
    let haystack = "a b c d e";
    let letters: Vec<_> = StrSplit::new(haystack, " ").collect();
    assert_eq!(letters, vec!["a", "b", "c", "d", "e"]);
}

#[test]
fn tail() {
    let haystack = "a b c d ";
    let letters: Vec<_> = StrSplit::new(haystack, " ").collect();
    assert_eq!(letters, vec!["a", "b", "c", "d", ""]);
}
