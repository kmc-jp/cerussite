#[derive(Debug)]
pub enum Expr {
    Assignment(Box<Assignment>),
}

#[derive(Debug)]
pub enum Assignment {
    Additive(Box<Additive>),
    Assign(Box<Unary>, Box<Assignment>),
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
    Paren(Box<Expr>),
}

use token::{Token, Tokens};

impl Expr {
    pub fn parse<'a>(tokens: &mut Tokens<'a>) -> Expr {
        Expr::Assignment(Box::new(Assignment::parse(tokens)))
    }

    pub fn gen_code(self, reg: usize) -> usize {
        match self {
            Expr::Assignment(assignment) => assignment.gen_code(reg),
        }
    }
}

/// <assignment> ::= <additive>
///                | <unary> OpAssign <assignment>
impl Assignment {
    pub fn parse<'a>(tokens: &mut Tokens<'a>) -> Assignment {
        let lhs = Additive::parse(tokens);
        match tokens.peek() {
            Some(Token::OpAssign) => {
                tokens.eat(Token::OpAssign);
                let rhs = Assignment::parse(tokens);
                let lhs = match lhs {
                    Additive::Multiplicative(lhs) => *lhs,
                    _ => panic!("left operand of assignment must be unary expression")
                };
                let lhs = match lhs {
                    Multiplicative::Unary(lhs) => *lhs,
                    _ => panic!("left operand of assignment must be unary expression")
                };
                Assignment::Assign(Box::new(lhs), Box::new(rhs))
            }
            _ => Assignment::Additive(Box::new(lhs))
        }
    }

    pub fn gen_code(self, reg: usize) -> usize {
        match self {
            Assignment::Additive(additive) => additive.gen_code(reg),
            Assignment::Assign(_unary, _assignment) => {
                unimplemented!()
            }
        }
    }
}

/// <additive> ::= <multiplicative>
///              | <multiplicative> <addive-dash>
/// <additive-dash> ::= OpAdd <multiplicative> <additive-dash>
///                   | OpSub <multiplicative> <additive-dash>
impl Additive {
    pub fn parse<'a>(tokens: &mut Tokens<'a>) -> Additive {
        let lhs = Multiplicative::parse(tokens);
        Additive::parse_additive_dash(Additive::Multiplicative(Box::new(lhs)), tokens)
    }

    fn parse_additive_dash<'a>(lhs: Additive, tokens: &mut Tokens<'a>) -> Additive {
        match tokens.peek() {
            Some(Token::OpAdd) => {
                tokens.eat(Token::OpAdd);
                let rhs = Multiplicative::parse(tokens);
                let additive = Additive::Add(Box::new(lhs), Box::new(rhs));
                Additive::parse_additive_dash(additive, tokens)
            }
            Some(Token::OpSub) => {
                tokens.eat(Token::OpSub);
                let rhs = Multiplicative::parse(tokens);
                let additive = Additive::Sub(Box::new(lhs), Box::new(rhs));
                Additive::parse_additive_dash(additive, tokens)
            }
            _ => lhs,
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
    pub fn parse<'a>(tokens: &mut Tokens<'a>) -> Multiplicative {
        let lhs = Unary::parse(tokens);
        Multiplicative::parse_multiplicative_dash(Multiplicative::Unary(Box::new(lhs)), tokens)
    }

    fn parse_multiplicative_dash<'a>(
        lhs: Multiplicative,
        tokens: &mut Tokens<'a>,
    ) -> Multiplicative {
        match tokens.peek() {
            Some(Token::OpMul) => {
                tokens.eat(Token::OpMul);
                let rhs = Unary::parse(tokens);
                let multiplicative = Multiplicative::Mul(Box::new(lhs), Box::new(rhs));
                Multiplicative::parse_multiplicative_dash(multiplicative, tokens)
            }
            Some(Token::OpDiv) => {
                tokens.eat(Token::OpDiv);
                let rhs = Unary::parse(tokens);
                let multiplicative = Multiplicative::Div(Box::new(lhs), Box::new(rhs));
                Multiplicative::parse_multiplicative_dash(multiplicative, tokens)
            }
            Some(Token::OpRem) => {
                tokens.eat(Token::OpRem);
                let rhs = Unary::parse(tokens);
                let multiplicative = Multiplicative::Rem(Box::new(lhs), Box::new(rhs));
                Multiplicative::parse_multiplicative_dash(multiplicative, tokens)
            }
            _ => lhs,
        }
    }

    pub fn gen_code(self, reg: usize) -> usize {
        match self {
            Multiplicative::Unary(unary) => unary.gen_code(reg),
            Multiplicative::Mul(multiplicative, unary) => {
                let lhs = multiplicative.gen_code(reg);
                let rhs = unary.gen_code(lhs + 1);
                println!("  %{} = mul i32 %{}, %{}", rhs + 1, lhs, rhs);
                rhs + 1
            }
            Multiplicative::Div(multiplicative, unary) => {
                let lhs = multiplicative.gen_code(reg);
                let rhs = unary.gen_code(lhs + 1);
                println!("  %{} = sdiv i32 %{}, %{}", rhs + 1, lhs, rhs);
                rhs + 1
            }
            Multiplicative::Rem(multiplicative, unary) => {
                let lhs = multiplicative.gen_code(reg);
                let rhs = unary.gen_code(lhs + 1);
                println!("  %{} = srem i32 %{}, %{}", rhs + 1, lhs, rhs);
                rhs + 1
            }
        }
    }
}

impl Unary {
    pub fn parse<'a>(tokens: &mut Tokens<'a>) -> Unary {
        match tokens.peek() {
            Some(Token::OpAdd) => {
                tokens.eat(Token::OpAdd);
                let unary = Unary::parse(tokens);
                Unary::UnaryPlus(Box::new(unary))
            }
            Some(Token::OpSub) => {
                tokens.eat(Token::OpSub);
                let unary = Unary::parse(tokens);
                Unary::UnaryMinus(Box::new(unary))
            }
            _ => {
                let primary = Primary::parse(tokens);
                Unary::Primary(Box::new(primary))
            }
        }
    }

    pub fn gen_code(self, reg: usize) -> usize {
        match self {
            Unary::Primary(primary) => primary.gen_code(reg),
            Unary::UnaryPlus(unary) => unary.gen_code(reg),
            Unary::UnaryMinus(unary) => {
                let reg = unary.gen_code(reg);
                println!("  %{} = sub i32 0, %{}", reg + 1, reg);
                reg + 1
            }
        }
    }
}

impl Primary {
    pub fn parse<'a>(tokens: &mut Tokens<'a>) -> Primary {
        match tokens.next() {
            Some(Token::Literal(n)) => {
                let constant = n.parse().expect("internal error: could not parse literal.");
                Primary::Constant(constant)
            }
            Some(Token::SyLPar) => {
                let expr = Expr::parse(tokens);
                tokens.eat_err(Token::SyRPar, "no matching parens for primary expression.");
                Primary::Paren(Box::new(expr))
            }
            other => {
                panic!("expected primary expression, found {:?}", other);
            }
        }
    }

    pub fn gen_code(self, reg: usize) -> usize {
        match self {
            Primary::Constant(n) => {
                println!("  %{} = add i32 {}, 0", reg, n);
                reg
            }
            Primary::Paren(expr) => expr.gen_code(reg),
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
