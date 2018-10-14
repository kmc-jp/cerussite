#![allow(dead_code)]
#![allow(unused_variables)]

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

/// <multiplicative> ::= <unary>
///                    | <unary> <multiplicative-dash>
/// <multiplicative-dash> ::= OpMul <unary> <multiplicative-dash>
///                         | OpDiv <unary> <multiplicative-dash>
impl Multiplicative {
    pub fn parse<'a>(tokens: Tokens<'a>) -> (Multiplicative, Tokens<'a>) {
        let (lhs, tokens) = Unary::parse(tokens);
        Multiplicative::parse_multiplicative_dash(Multiplicative::Unary(Box::new(lhs)), tokens)
    }

    fn parse_multiplicative_dash<'a>(
        lhs: Multiplicative,
        tokens: Tokens<'a>,
    ) -> (Multiplicative, Tokens<'a>) {
        match tokens.iter().next() {
            Some(Token::OpMul) => {
                let (rhs, tokens) = Unary::parse(&tokens[1..]);
                let multiplicative = Multiplicative::Mul(Box::new(lhs), Box::new(rhs));
                Multiplicative::parse_multiplicative_dash(multiplicative, tokens)
            }
            Some(Token::OpDiv) => {
                let (rhs, tokens) = Unary::parse(&tokens[1..]);
                let multiplicative = Multiplicative::Div(Box::new(lhs), Box::new(rhs));
                Multiplicative::parse_multiplicative_dash(multiplicative, tokens)
            }
            _ => (lhs, tokens),
        }
    }
}

impl Unary {
    pub fn parse<'a>(tokens: Tokens<'a>) -> (Unary, Tokens<'a>) {
        let (primary, tokens) = Primary::parse(tokens);
        (Unary::Primary(Box::new(primary)), tokens)
    }
}

impl Primary {
    pub fn parse<'a>(tokens: Tokens<'a>) -> (Primary, Tokens<'a>) {
        assert!(
            !tokens.is_empty(),
            "expected primary expression, found nothing."
        );

        match tokens[0] {
            Token::Literal(n) => {
                let constant = n.parse().expect("internal error: could not parse literal.");
                (Primary::Constant(constant), &tokens[1..])
            }
            Token::SyLPar => {
                let (expr, tokens) = Expr::parse(&tokens[1..]);
                assert_eq!(tokens[0], Token::SyRPar);
                (Primary::Paren(Box::new(expr)), &tokens[1..])
            }
            _ => {
                panic!("expected primary expression, found {:?}", tokens);
            }
        }
    }
}
