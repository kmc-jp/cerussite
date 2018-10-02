use std::iter::Peekable;
use std::str::CharIndices;

pub struct Tokenizer<'a> {
    source: &'a str,
    chars: Peekable<CharIndices<'a>>,
}

impl<'a> Tokenizer<'a> {
    pub fn from_source(source: &'a str) -> Tokenizer {
        Tokenizer {
            source: source,
            chars: source.char_indices().peekable(),
        }
    }
}

impl<'a> Iterator for Tokenizer<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        // inside loop, find chars between the current position and the next
        // whitespace char. if that string's length is >= 1, this returns it
        // as a token. if it can't found any chars (for example, when several
        // whitespace chars appear sequencially), loop continues and retries
        // this operation.  if all all source code consumed, this returns
        // None.
        loop {
            // if whole source code has been read, returns None.
            // otherwise, store the current position.
            let first = match self.chars.peek() {
                None => return None,
                Some(&(first, _)) => first,
            };

            // find the last char before a whitespace char
            let last_char = self.chars.find(|(_, ch)| ch.is_whitespace());

            match last_char {
                Some((last, _)) if first < last => {
                    // we can get token here, returns that.
                    return Some(&self.source[first..last]);
                }
                None => {
                    // consumed all source code.
                    // there are at least one character, so we can safely
                    // return this (no need to concern blank string)
                    return Some(&self.source[first..]);
                }
                _ => {}
            }

            // otherwise, loop continues and retries this process.
        }
    }
}
