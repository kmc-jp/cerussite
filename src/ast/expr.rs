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
    Paren(Box<Expr>),
}

use token::{Token, Tokens};

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

    fn parse_additive_dash<'a>(lhs: Additive, mut tokens: Tokens<'a>) -> (Additive, Tokens<'a>) {
        match tokens.peek() {
            Some(Token::OpAdd) => {
                tokens.eat(Token::OpAdd);
                let (rhs, tokens) = Multiplicative::parse(tokens);
                let additive = Additive::Add(Box::new(lhs), Box::new(rhs));
                Additive::parse_additive_dash(additive, tokens)
            }
            Some(Token::OpSub) => {
                tokens.eat(Token::OpSub);
                let (rhs, tokens) = Multiplicative::parse(tokens);
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
        mut tokens: Tokens<'a>,
    ) -> (Multiplicative, Tokens<'a>) {
        match tokens.peek() {
            Some(Token::OpMul) => {
                tokens.eat(Token::OpMul);
                let (rhs, tokens) = Unary::parse(tokens);
                let multiplicative = Multiplicative::Mul(Box::new(lhs), Box::new(rhs));
                Multiplicative::parse_multiplicative_dash(multiplicative, tokens)
            }
            Some(Token::OpDiv) => {
                tokens.eat(Token::OpDiv);
                let (rhs, tokens) = Unary::parse(tokens);
                let multiplicative = Multiplicative::Div(Box::new(lhs), Box::new(rhs));
                Multiplicative::parse_multiplicative_dash(multiplicative, tokens)
            }
            Some(Token::OpRem) => {
                tokens.eat(Token::OpRem);
                let (rhs, tokens) = Unary::parse(tokens);
                let multiplicative = Multiplicative::Rem(Box::new(lhs), Box::new(rhs));
                Multiplicative::parse_multiplicative_dash(multiplicative, tokens)
            }
            _ => (lhs, tokens),
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
    pub fn parse<'a>(mut tokens: Tokens<'a>) -> (Unary, Tokens<'a>) {
        match tokens.peek() {
            Some(Token::OpAdd) => {
                tokens.eat(Token::OpAdd);
                let (unary, tokens) = Unary::parse(tokens);
                (Unary::UnaryPlus(Box::new(unary)), tokens)
            }
            Some(Token::OpSub) => {
                tokens.eat(Token::OpSub);
                let (unary, tokens) = Unary::parse(tokens);
                (Unary::UnaryMinus(Box::new(unary)), tokens)
            }
            _ => {
                let (primary, tokens) = Primary::parse(tokens);
                (Unary::Primary(Box::new(primary)), tokens)
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
    pub fn parse<'a>(mut tokens: Tokens<'a>) -> (Primary, Tokens<'a>) {
        match tokens.next() {
            Some(Token::Literal(n)) => {
                let constant = n.parse().expect("internal error: could not parse literal.");
                (Primary::Constant(constant), tokens)
            }
            Some(Token::SyLPar) => {
                let (expr, mut tokens) = Expr::parse(tokens);
                tokens.eat_err(Token::SyRPar, "no matching parens for primary expression.");
                (Primary::Paren(Box::new(expr)), tokens)
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
    use super::Expr;
    use token::{Token, Tokens};

    #[test]
    fn it_works() {
        println!("{:?}", Expr::parse(Tokens::new(&[Token::Literal("42")])));
        println!(
            "{:?}",
            Expr::parse(Tokens::new(&[
                Token::Literal("40"),
                Token::OpAdd,
                Token::Literal("2")
            ]))
        );
        println!(
            "{:?}",
            Expr::parse(Tokens::new(&[
                Token::Literal("42"),
                Token::OpAdd,
                Token::Literal("3"),
                Token::OpMul,
                Token::Literal("7")
            ]))
        );
        println!(
            "{:?}",
            Expr::parse(Tokens::new(&[
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
            ]))
        );
    }
}
