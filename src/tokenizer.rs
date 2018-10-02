use std::marker::PhantomData;

pub struct Tokenizer<'a> {
    // under construction
    _dummy: PhantomData<&'a str>,
}

impl<'a> Tokenizer<'a> {
    pub fn from_source(source: &'a str) -> impl Iterator<Item = &'a str> {
        // currently the supported delimiters are [' ', '\n'].
        source
            .split(' ')
            .flat_map(|x| x.split('\n'))
            .map(str::trim)
            .filter(|&x| x != "")
    }
}
