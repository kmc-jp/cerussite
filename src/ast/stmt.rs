use super::expr::Expr;
use token::{Token, Tokens};

#[derive(Debug)]
pub enum Stmt {
    Jump(Box<Jump>),
}

#[derive(Debug)]
pub enum Jump {
    Return(Box<Expr>),
}

impl Stmt {
    pub fn parse<'a>(tokens: Tokens<'a>) -> (Stmt, Tokens<'a>) {
        let (jump, tokens) = Jump::parse(tokens);
        (Stmt::Jump(Box::new(jump)), tokens)
    }

    pub fn gen_code(self, reg: usize) -> usize {
        match self {
            Stmt::Jump(jump) => jump.gen_code(reg),
        }
    }
}

impl Jump {
    pub fn parse<'a>(tokens: Tokens<'a>) -> (Jump, Tokens<'a>) {
        match tokens[0] {
            Token::KwReturn => {
                let (expr, tokens) = Expr::parse(&tokens[1..]);
                assert_eq!(tokens[0], Token::SySemicolon);
                (Jump::Return(Box::new(expr)), &tokens[1..])
            }
            _ => panic!("expected jump statement, found other."),
        }
    }

    pub fn gen_code(self, reg: usize) -> usize {
        match self {
            Jump::Return(expr) => {
                let reg = expr.gen_code(reg);
                println!("  ret i32 %{}", reg);
                reg + 1
            }
        }
    }
}
