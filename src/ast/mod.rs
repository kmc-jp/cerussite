pub mod expr;
pub mod stmt;

use self::stmt::Stmt;
use token::Tokens;

pub enum Ast {
    Stmt(Stmt),
}

impl Ast {
    pub fn parse<'a>(tokens: Tokens<'a>) -> Ast {
        let (stmt, tokens) = Stmt::parse(tokens);
        if !tokens.is_empty() {
            panic!("invalid tokens: {:?}", tokens);
        }
        Ast::Stmt(stmt)
    }

    pub fn gen_code(self) -> usize {
        match self {
            Ast::Stmt(stmt) => stmt.gen_code(1),
        }
    }
}
