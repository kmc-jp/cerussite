pub enum Expr {
    Additive(Box<Additive>),
}

pub enum Additive {
    Multiplicative(Box<Multiplicative>),
    Add(Box<Additive>, Box<Multiplicative>),
    Sub(Box<Additive>, Box<Multiplicative>),
}

pub enum Multiplicative {
    Unary(Box<Unary>),
    Mul(Box<Multiplicative>, Box<Unary>),
    Div(Box<Multiplicative>, Box<Unary>),
}

pub enum Unary {
    Primary(Box<Primary>),
}

pub enum Primary {
    Constant(i32),
    Paren(Box<Expr>),
}

use token::Token;
use token::Tokens;

impl Expr {
    pub fn parse<'a>(tokens: Tokens<'a>) -> (Expr, Tokens<'a>) {
        let (additive, tokens) = Additive::parse(tokens);
        (Expr::Additive(Box::new(additive)), tokens)
    }
}

/// <additive> ::= <multiplicative>
///              | <multiplicative> <addive-dash>
/// <additive-dash> ::= OpAdd <multiplicative> <additive-dash>
///                   | OpSub <multiplicative> <additive-dash>
impl Additive {
    pub fn parse<'a>(tokens: Tokens<'a>) -> (Additive, Tokens<'a>) {
        let (lhs, tokens) = Multiplicative::parse(tokens);
        Additive::parse_additive_dash(Additive::Multiplicative(Box::new(lhs)), tokens)
    }

    fn parse_additive_dash<'a>(lhs: Additive, tokens: Tokens<'a>) -> (Additive, Tokens<'a>) {
        match tokens.iter().next() {
            Some(Token::OpAdd) => {
                let (rhs, tokens) = Multiplicative::parse(&tokens[1..]);
                let additive = Additive::Add(Box::new(lhs), Box::new(rhs));
                Additive::parse_additive_dash(additive, tokens)
            }
            Some(Token::OpSub) => {
                let (rhs, tokens) = Multiplicative::parse(&tokens[1..]);
                let additive = Additive::Sub(Box::new(lhs), Box::new(rhs));
                Additive::parse_additive_dash(additive, tokens)
            }
            _ => (lhs, tokens),
        }
    }
}

impl Multiplicative {
    pub fn parse<'a>(tokens: Tokens<'a>) -> (Multiplicative, Tokens<'a>) {
        unimplemented!();
    }
}

impl Unary {
    pub fn parse<'a>(tokens: Tokens<'a>) -> (Unary, Tokens<'a>) {
        unimplemented!();
    }
}

impl Primary {
    pub fn parse<'a>(tokens: Tokens<'a>) -> (Primary, Tokens<'a>) {
        unimplemented!();
    }
}
