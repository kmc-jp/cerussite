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

use token::Tokens;

impl Expr {
    pub fn parse<'a>(tokens: Tokens<'a>) -> Expr {
        unimplemented!();
    }
}
