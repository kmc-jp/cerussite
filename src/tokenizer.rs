use itertools::Itertools;

use std::iter::Peekable;

/// split the source code into tokens
pub struct Tokenizer<'a> {
    /// entire source code
    source: &'a str,

    /// (starting, ending, character)
    /// this is Peekable to use peeking_take_while().
    chars: Peekable<Box<dyn Iterator<Item = (usize, usize, char)> + 'a>>,
}

impl<'a> Tokenizer<'a> {
    pub fn from_source(source: &'a str) -> Tokenizer {
        Tokenizer {
            source: source,
            chars: gen_bid_char_indices(source),
        }
    }
}

/// generate char_indices that having (starting, ending, char)
fn gen_bid_char_indices<'a>(
    source: &'a str,
) -> Peekable<Box<dyn Iterator<Item = (usize, usize, char)> + 'a>> {
    let ending_indices = source
        .char_indices()
        .map(|(pos, _)| pos)
        .skip(1)
        .chain(Some(source.len()).into_iter());

    let bid_char_indices = source
        .char_indices()
        .zip(ending_indices)
        .map(|((starting, ch), ending)| (starting, ending, ch));

    let bid_char_indices: Box<dyn Iterator<Item = (usize, usize, char)>> =
        Box::new(bid_char_indices);

    bid_char_indices.peekable()
}

impl<'a> Iterator for Tokenizer<'a> {
    type Item = &'a str;

    /// find a next token starting from current position (if it is whitespace, skips them and from
    /// a next non-whitespace character).  if entire source code consumed, this returns None.
    fn next(&mut self) -> Option<Self::Item> {
        // skip whitespace
        self.chars
            .peeking_take_while(|&(_, _, ch)| ch.is_whitespace())
            .for_each(drop);

        // get first char position. if there isn't, returns None.
        let first = self.chars.peek().map(|&(first, _, _)| first)?;

        // to avoid borrowing self in closure
        let source = self.source;

        // find longest token
        let last = self
            .chars
            .peeking_take_while(|&(_, pos, _)| is_valid_token(&source[first..pos]))
            .fold(first, |_, (_, pos, _)| pos);

        Some(&source[first..last])
    }
}

/// check if given `s` is valid token or not.
/// TODO: under construction.
fn is_valid_token(s: &str) -> bool {
    true
}
