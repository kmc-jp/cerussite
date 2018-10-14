#![allow(dead_code)]
#![allow(unused_variables)]

#[derive(Debug)]
pub enum Expr {
    Additive(Box<Additive>),
}

#[derive(Debug)]
pub enum Additive {
    Multiplicative(Box<Multiplicative>),
    Add(Box<Additive>, Box<Multiplicative>),
    Sub(Box<Additive>, Box<Multiplicative>),
}

#[derive(Debug)]
pub enum Multiplicative {
    Unary(Box<Unary>),
    Mul(Box<Multiplicative>, Box<Unary>),
    Div(Box<Multiplicative>, Box<Unary>),
}

#[derive(Debug)]
pub enum Unary {
    Primary(Box<Primary>),
}

#[derive(Debug)]
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

    pub fn gen_code(self, reg: usize) -> usize {
        match self {
            Expr::Additive(additive) => additive.gen_code(reg),
        }
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

    pub fn gen_code(self, reg: usize) -> usize {
        match self {
            Additive::Multiplicative(multiplicative) => multiplicative.gen_code(reg),
            Additive::Add(additive, multiplicative) => {
                let lhs = additive.gen_code(reg);
                let rhs = multiplicative.gen_code(lhs + 1);
                println!("  %{} = add i32 %{}, %{}", rhs + 1, lhs, rhs);
                rhs + 1
            }
            Additive::Sub(additive, multiplicative) => {
                let lhs = additive.gen_code(reg);
                let rhs = multiplicative.gen_code(lhs + 1);
                println!("  %{} = sub i32 %{}, %{}", rhs + 1, lhs, rhs);
                rhs + 1
            }
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

    pub fn gen_code(self, reg: usize) -> usize {
        unimplemented!();
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

#[cfg(test)]
mod tests {
    use super::Expr;
    use token::Token;

    #[test]
    fn it_works() {
        println!("{:?}", Expr::parse(&[Token::Literal("42")]));
        println!(
            "{:?}",
            Expr::parse(&[Token::Literal("40"), Token::OpAdd, Token::Literal("2")])
        );
        println!(
            "{:?}",
            Expr::parse(&[
                Token::Literal("42"),
                Token::OpAdd,
                Token::Literal("3"),
                Token::OpMul,
                Token::Literal("7")
            ])
        );
        println!(
            "{:?}",
            Expr::parse(&[
                Token::Literal("42"),
                Token::OpAdd,
                Token::SyLPar,
                Token::Literal("30"),
                Token::OpSub,
                Token::SyLPar,
                Token::Literal("30"),
                Token::OpSub,
                Token::Literal("15"),
                Token::SyRPar,
                Token::SyRPar
            ])
        );
    }
}
