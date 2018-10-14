pub mod expr;

use self::expr::Expr;
use token::Token;

pub enum Ast {
    Expr(Expr),
}

impl Ast {
    pub fn parse<'a>(tokens: &'a [Token<'a>]) -> Ast {
        let expr = Expr::parse(tokens);
        Ast::Expr(expr)
    }
}
