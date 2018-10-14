pub mod expr;

use self::expr::Expr;
use token::Tokens;

pub enum Ast {
    Expr(Expr),
}

impl Ast {
    pub fn parse<'a>(tokens: Tokens<'a>) -> Ast {
        let (expr, tokens) = Expr::parse(tokens);
        if !tokens.is_empty() {
            panic!("invalid tokens: {:?}", tokens);
        }
        Ast::Expr(expr)
    }
}
