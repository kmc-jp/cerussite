/*
 * BNF which expression parser can accept for now
 *
 *
 * <expr> ::= <additive>
 *
 * <additive> ::= <multiplicative>
 *              | <additive> + <multiplicative>
 *              | <additive> - <multiplicative>
 *
 * <multiplicative> ::= <unary>
 *                    | <multiplicative> * <unary>
 *                    | <multiplicative> / <unary>
 *                    | <multiplicative> % <unary>
 *
 * <unary> ::= <primary>
 *           | + <unary>
 *           | - <unary>
 *
 * <primary> ::= <constant>
 *             | <identifier>
 *             | ( <expr> )
 */
use super::code_gen_state::CodeGenState;
use super::{ParseError, Result};

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
    Rem(Box<Multiplicative>, Box<Unary>),
}

#[derive(Debug)]
pub enum Unary {
    Primary(Box<Primary>),
    UnaryPlus(Box<Unary>),
    UnaryMinus(Box<Unary>),
}

#[derive(Debug)]
pub enum Primary {
    Constant(i32),
    Identifier(String),
    Paren(Box<Expr>),
}

use token::{Token, Tokens};

impl Expr {
    pub fn parse<'a>(tokens: &mut Tokens<'a>) -> Result<Expr> {
        Ok(Expr::Additive(Box::new(Additive::parse(tokens)?)))
    }

    pub fn gen_code(self, state: &mut CodeGenState) -> usize {
        match self {
            Expr::Additive(additive) => additive.gen_code(state),
        }
    }
}

/// <additive> ::= <multiplicative>
///              | <multiplicative> <addive-dash>
/// <additive-dash> ::= OpAdd <multiplicative> <additive-dash>
///                   | OpSub <multiplicative> <additive-dash>
impl Additive {
    pub fn parse<'a>(tokens: &mut Tokens<'a>) -> Result<Additive> {
        let lhs = Multiplicative::parse(tokens)?;
        Additive::parse_additive_dash(Additive::Multiplicative(Box::new(lhs)), tokens)
    }

    fn parse_additive_dash<'a>(lhs: Additive, tokens: &mut Tokens<'a>) -> Result<Additive> {
        match tokens.peek() {
            Some(Token::OpAdd) => {
                tokens.eat(Token::OpAdd);
                let rhs = Multiplicative::parse(tokens)?;
                let additive = Additive::Add(Box::new(lhs), Box::new(rhs));
                Additive::parse_additive_dash(additive, tokens)
            }
            Some(Token::OpSub) => {
                tokens.eat(Token::OpSub);
                let rhs = Multiplicative::parse(tokens)?;
                let additive = Additive::Sub(Box::new(lhs), Box::new(rhs));
                Additive::parse_additive_dash(additive, tokens)
            }
            _ => Ok(lhs),
        }
    }

    pub fn gen_code(self, state: &mut CodeGenState) -> usize {
        match self {
            Additive::Multiplicative(multiplicative) => multiplicative.gen_code(state),
            Additive::Add(additive, multiplicative) => {
                let lhs = additive.gen_code(state);
                let rhs = multiplicative.gen_code(state);
                let reg = state.next_reg();
                println!("  %{} = add i32 %{}, %{}", reg, lhs, rhs);
                reg
            }
            Additive::Sub(additive, multiplicative) => {
                let lhs = additive.gen_code(state);
                let rhs = multiplicative.gen_code(state);
                let reg = state.next_reg();
                println!("  %{} = sub i32 %{}, %{}", reg, lhs, rhs);
                reg
            }
        }
    }
}

/// <multiplicative> ::= <unary>
///                    | <unary> <multiplicative-dash>
/// <multiplicative-dash> ::= OpMul <unary> <multiplicative-dash>
///                         | OpDiv <unary> <multiplicative-dash>
impl Multiplicative {
    pub fn parse<'a>(tokens: &mut Tokens<'a>) -> Result<Multiplicative> {
        let lhs = Unary::parse(tokens)?;
        Multiplicative::parse_multiplicative_dash(Multiplicative::Unary(Box::new(lhs)), tokens)
    }

    fn parse_multiplicative_dash<'a>(
        lhs: Multiplicative,
        tokens: &mut Tokens<'a>,
    ) -> Result<Multiplicative> {
        match tokens.peek() {
            Some(Token::OpMul) => {
                tokens.eat(Token::OpMul);
                let rhs = Unary::parse(tokens)?;
                let multiplicative = Multiplicative::Mul(Box::new(lhs), Box::new(rhs));
                Multiplicative::parse_multiplicative_dash(multiplicative, tokens)
            }
            Some(Token::OpDiv) => {
                tokens.eat(Token::OpDiv);
                let rhs = Unary::parse(tokens)?;
                let multiplicative = Multiplicative::Div(Box::new(lhs), Box::new(rhs));
                Multiplicative::parse_multiplicative_dash(multiplicative, tokens)
            }
            Some(Token::OpRem) => {
                tokens.eat(Token::OpRem);
                let rhs = Unary::parse(tokens)?;
                let multiplicative = Multiplicative::Rem(Box::new(lhs), Box::new(rhs));
                Multiplicative::parse_multiplicative_dash(multiplicative, tokens)
            }
            _ => Ok(lhs),
        }
    }

    pub fn gen_code(self, state: &mut CodeGenState) -> usize {
        match self {
            Multiplicative::Unary(unary) => unary.gen_code(state),
            Multiplicative::Mul(multiplicative, unary) => {
                let lhs = multiplicative.gen_code(state);
                let rhs = unary.gen_code(state);
                let reg = state.next_reg();
                println!("  %{} = mul i32 %{}, %{}", reg, lhs, rhs);
                reg
            }
            Multiplicative::Div(multiplicative, unary) => {
                let lhs = multiplicative.gen_code(state);
                let rhs = unary.gen_code(state);
                let reg = state.next_reg();
                println!("  %{} = sdiv i32 %{}, %{}", reg, lhs, rhs);
                reg
            }
            Multiplicative::Rem(multiplicative, unary) => {
                let lhs = multiplicative.gen_code(state);
                let rhs = unary.gen_code(state);
                let reg = state.next_reg();
                println!("  %{} = srem i32 %{}, %{}", reg, lhs, rhs);
                reg
            }
        }
    }
}

impl Unary {
    pub fn parse<'a>(tokens: &mut Tokens<'a>) -> Result<Unary> {
        match tokens.peek() {
            Some(Token::OpAdd) => {
                tokens.eat(Token::OpAdd);
                let unary = Unary::parse(tokens)?;
                Ok(Unary::UnaryPlus(Box::new(unary)))
            }
            Some(Token::OpSub) => {
                tokens.eat(Token::OpSub);
                let unary = Unary::parse(tokens)?;
                Ok(Unary::UnaryMinus(Box::new(unary)))
            }
            _ => {
                let primary = Primary::parse(tokens)?;
                Ok(Unary::Primary(Box::new(primary)))
            }
        }
    }

    pub fn gen_code(self, state: &mut CodeGenState) -> usize {
        match self {
            Unary::Primary(primary) => primary.gen_code(state),
            Unary::UnaryPlus(unary) => unary.gen_code(state),
            Unary::UnaryMinus(unary) => {
                let unary = unary.gen_code(state);
                let reg = state.next_reg();
                println!("  %{} = sub i32 0, %{}", reg, unary);
                reg
            }
        }
    }
}

impl Primary {
    pub fn parse<'a>(tokens: &mut Tokens<'a>) -> Result<Primary> {
        match tokens.next() {
            Some(Token::Literal(n)) => {
                let constant = n.parse().expect("internal error: could not parse literal.");
                Ok(Primary::Constant(constant))
            }
            Some(Token::SyLPar) => {
                let expr = Expr::parse(tokens)?;
                tokens.eat_err(Token::SyRPar, "no matching parens for primary expression.");
                Ok(Primary::Paren(Box::new(expr)))
            }
            Some(Token::Ident(ident)) => Ok(Primary::Identifier(ident.into())),
            other => Err(ParseError::Unexpected {
                expected: "primary expression".into(),
                found: format!("{:?}", other),
            }),
        }
    }

    pub fn gen_code(self, state: &mut CodeGenState) -> usize {
        match self {
            Primary::Constant(n) => {
                let reg = state.next_reg();
                println!("  %{} = add i32 {}, 0", reg, n);
                reg
            }
            Primary::Identifier(ident) => {
                let reg = state.next_reg();
                let var = state
                    .vars
                    .get(&ident)
                    .expect(&format!("undeclared identifier: `{}`", ident));
                println!(
                    "  %{} = load {}, {}* %{}, align {}",
                    reg, var.tyir, var.tyir, var.reg, var.align
                );
                reg
            }
            Primary::Paren(expr) => expr.gen_code(state),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use token::{Token, Tokens};

    #[test]
    fn parse_expr() {
        let tests = vec![
            Tokens::new(&[Token::Literal("42")]),
            Tokens::new(&[Token::Literal("40"), Token::OpAdd, Token::Literal("2")]),
            Tokens::new(&[
                Token::Literal("42"),
                Token::OpAdd,
                Token::Literal("3"),
                Token::OpMul,
                Token::Literal("7"),
            ]),
            Tokens::new(&[
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
                Token::SyRPar,
            ]),
        ];

        for mut tokens in tests {
            println!("{:?}", Expr::parse(&mut tokens));
            assert!(tokens.is_empty());
        }
    }
}
