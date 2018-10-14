pub mod expr;

use self::expr::Expr;
use token::Tokens;

pub enum Ast {
    Expr(Expr),
}

impl Ast {
    pub fn parse<'a>(tokens: Tokens<'a>) -> Ast {
        let expr = Expr::parse(tokens);
        Ast::Expr(expr)
    }
}
