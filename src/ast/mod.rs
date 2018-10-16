pub mod code_gen_state;
pub mod expr;
pub mod stmt;

use self::code_gen_state::CodeGenState;
use self::stmt::Stmt;
use token::Tokens;

use std::{error, fmt, result};

pub type Result<T> = result::Result<T, ParseError>;

#[derive(Debug)]
pub enum ParseError {}

impl fmt::Display for ParseError {
    fn fmt(&self, b: &mut fmt::Formatter) -> fmt::Result {
        write!(b, "{:?}", *self)
    }
}

impl error::Error for ParseError {}

pub enum Ast {
    Stmt(Stmt),
}

impl Ast {
    pub fn parse<'a>(mut tokens: Tokens<'a>) -> Ast {
        let stmt = Stmt::parse(&mut tokens);
        if !tokens.is_empty() {
            panic!("invalid tokens: {:?}", tokens);
        }
        Ast::Stmt(stmt)
    }

    pub fn gen_code(self) {
        let mut state = CodeGenState::new();
        match self {
            Ast::Stmt(stmt) => stmt.gen_code(&mut state),
        }
    }
}
