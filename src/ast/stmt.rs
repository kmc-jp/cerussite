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
    pub fn parse<'a>(tokens: &mut Tokens<'a>) -> Compound {
        match tokens.next() {
            Some(Token::SyLBrace) => {
                let mut stmts = Vec::new();
                loop {
                    stmts.push(Stmt::parse(tokens));
                    if let Some(Token::SyRBrace) = tokens.peek() {
                        tokens.eat(Token::SyRBrace);
                        break;
                    }
                }
                Compound { stmts }
            }
            other => {
                panic!("expected compound statement (`{{`), found {:?}", other);
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
    pub fn parse<'a>(tokens: &mut Tokens<'a>) -> Stmt {
        match tokens.peek() {
            Some(Token::SyLBrace) => Stmt::Compound(Box::new(Compound::parse(tokens))),
            _ => Stmt::Jump(Box::new(Jump::parse(tokens))),
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
    pub fn parse<'a>(tokens: &mut Tokens<'a>) -> Jump {
        match tokens.next() {
            Some(Token::KwReturn) => {
                let expr = Expr::parse(tokens);
                tokens.eat_err(
                    Token::SySemicolon,
                    "missing semicolon after jump statement.",
                );
                Jump::Return(Box::new(expr))
            }
            other => panic!("expected jump statement, found {:?}", other),
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_return() {
        let tokens = &mut Tokens::new(&[Token::KwReturn, Token::Literal("42"), Token::SySemicolon]);
        let _ = Stmt::parse(tokens);
    }
}
