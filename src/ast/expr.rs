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
    pub fn parse<'a>(tokens: Tokens<'a>) -> (Expr, Tokens<'a>) {
        unimplemented!();
    }
}

impl Additive {
    pub fn parse<'a>(tokens: Tokens<'a>) -> (Additive, Tokens<'a>) {
        unimplemented!();
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
