use regex::Regex;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum Token<'a> {
    TyInt,
    TyVoid,

    KwReturn,

    OpAdd,
    OpSub,
    OpMul,
    OpDiv,

    SyLPar,
    SyRPar,
    SyLBrace,
    SyRBrace,
    SySemicolon,

    Ident(&'a str),
    Literal(&'a str),
}

lazy_static! {
    static ref RE_IDENT: Regex = Regex::new("^[a-zA-Z_][a-zA-Z0-9_]*$").unwrap();
    static ref RE_LITERAL: Regex = Regex::new("^[0-9]+$").unwrap();
}

impl<'a> Token<'a> {
    pub fn from_str(token_str: &'a str) -> Option<Token<'a>> {
        let maybe_token = match token_str {
            "int" => Some(Token::TyInt),
            "void" => Some(Token::TyVoid),

            "return" => Some(Token::KwReturn),

            "+" => Some(Token::OpAdd),
            "-" => Some(Token::OpSub),
            "*" => Some(Token::OpMul),
            "/" => Some(Token::OpDiv),

            "(" => Some(Token::SyLPar),
            ")" => Some(Token::SyRPar),
            "{" => Some(Token::SyLBrace),
            "}" => Some(Token::SyRBrace),
            ";" => Some(Token::SySemicolon),

            _ => None,
        };

        if maybe_token.is_some() {
            return maybe_token;
        }

        if RE_IDENT.is_match(token_str) {
            return Some(Token::Ident(token_str));
        }

        if RE_LITERAL.is_match(token_str) {
            return Some(Token::Literal(token_str));
        }

        None
    }
}

impl<'a> Token<'a> {
    pub fn unwrap_literal(&self) -> &str {
        match *self {
            Token::Literal(n) => n,
            _ => panic!("expected literal, found {:?}", self),
        }
    }
}
