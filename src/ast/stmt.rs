use super::expr::Expr;
use token::{Token, Tokens};

#[derive(Debug)]
pub struct Compound {
    stmts: Vec<Stmt>,
}

#[derive(Debug)]
pub enum Stmt {
    Compound(Box<Compound>),
    Jump(Box<Jump>),
}

#[derive(Debug)]
pub enum Jump {
    Return(Box<Expr>),
}

impl Compound {
    pub fn parse<'a>(mut tokens: Tokens<'a>) -> (Compound, Tokens<'a>) {
        match tokens[0] {
            Token::SyLBrace => {
                tokens = &tokens[1..];
                let mut stmts = Vec::new();
                loop {
                    eprintln!("rest of tokens: {:?}", tokens);
                    let (stmt, toks) = Stmt::parse(tokens);
                    tokens = toks;
                    stmts.push(stmt);
                    match tokens[0] {
                        Token::SyRBrace => {
                            tokens = &tokens[1..];
                            break;
                        }
                        _ => continue,
                    }
                }
                (Compound { stmts }, tokens)
            }
            _ => {
                panic!("expected compound statement (`{{`), found {:?}", tokens[0]);
            }
        }
    }

    pub fn gen_code(self, mut reg: usize) -> usize {
        for stmt in self.stmts {
            reg = stmt.gen_code(reg);
            reg += 1;
        }
        reg
    }
}

impl Stmt {
    pub fn parse<'a>(tokens: Tokens<'a>) -> (Stmt, Tokens<'a>) {
        match tokens[0] {
            Token::SyLBrace => {
                let (compound, tokens) = Compound::parse(tokens);
                (Stmt::Compound(Box::new(compound)), tokens)
            }
            _ => {
                let (jump, tokens) = Jump::parse(tokens);
                (Stmt::Jump(Box::new(jump)), tokens)
            }
        }
    }

    pub fn gen_code(self, reg: usize) -> usize {
        match self {
            Stmt::Compound(compound) => compound.gen_code(reg),
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
