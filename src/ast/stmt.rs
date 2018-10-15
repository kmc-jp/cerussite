use super::expr::{Additive as AdditiveExpr, Expr};
use token::{Token, Tokens};

#[derive(Debug)]
pub struct Compound {
    decls: Vec<Decl>,
    stmts: Vec<Stmt>,
}

#[derive(Debug)]
pub enum Stmt {
    Compound(Box<Compound>),
    Jump(Box<Jump>),
}

#[derive(Debug)]
pub struct Decl {
    specs: Vec<DeclSpecifier>,
    inits: Vec<InitDeclarator>,
}

#[derive(Debug)]
pub enum DeclSpecifier {
    TypeSpecifier(Box<TypeSpecifier>),
}

#[derive(Debug)]
pub enum TypeSpecifier {
    Int,
}

#[derive(Debug)]
pub enum InitDeclarator {
    Declarator(Box<Declarator>),
    DeclaratorWithValue(Box<Declarator>, Box<Initializer>),
}

#[derive(Debug)]
pub enum Declarator {
    Identifier(String),
}

#[derive(Debug)]
pub enum Initializer {
    Additive(Box<AdditiveExpr>),
}

#[derive(Debug)]
pub enum Jump {
    Return(Box<Expr>),
}

impl Compound {
    pub fn parse<'a>(tokens: &mut Tokens<'a>) -> Compound {
        match tokens.next() {
            Some(Token::SyLBrace) => {
                let mut decls = Vec::new();
                while Decl::is_your_job(tokens) {
                    decls.push(Decl::parse(tokens));
                }

                let mut stmts = Vec::new();
                loop {
                    stmts.push(Stmt::parse(tokens));
                    if let Some(Token::SyRBrace) = tokens.peek() {
                        tokens.eat(Token::SyRBrace);
                        break;
                    }
                }
                Compound { stmts, decls }
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

impl Decl {
    pub fn is_your_job<'a>(tokens: &Tokens<'a>) -> bool {
        DeclSpecifier::is_your_job(tokens)
    }

    pub fn parse<'a>(tokens: &mut Tokens<'a>) -> Decl {
        let mut specs = Vec::new();
        while DeclSpecifier::is_your_job(tokens) {
            specs.push(DeclSpecifier::parse(tokens));
        }

        let mut inits = Vec::new();
        loop {
            if let Some(Token::SySemicolon) = tokens.peek() {
                tokens.eat(Token::SySemicolon);
                break;
            }
            inits.push(InitDeclarator::parse(tokens));
        }

        Decl { specs, inits }
    }
}

impl DeclSpecifier {
    pub fn is_your_job<'a>(tokens: &Tokens<'a>) -> bool {
        TypeSpecifier::is_your_job(tokens)
    }

    pub fn parse<'a>(tokens: &mut Tokens<'a>) -> DeclSpecifier {
        DeclSpecifier::TypeSpecifier(Box::new(TypeSpecifier::parse(tokens)))
    }
}

impl TypeSpecifier {
    pub fn is_your_job<'a>(tokens: &Tokens<'a>) -> bool {
        tokens.peek() == Some(Token::TyInt)
    }

    pub fn parse<'a>(tokens: &mut Tokens<'a>) -> TypeSpecifier {
        match tokens.next() {
            Some(Token::TyInt) => TypeSpecifier::Int,
            other => panic!("expected type-specifier, found {:?}", other),
        }
    }
}

impl InitDeclarator {
    pub fn parse<'a>(tokens: &mut Tokens<'a>) -> InitDeclarator {
        let declarator = Declarator::parse(tokens);
        match tokens.peek() {
            Some(Token::OpAssign) => {
                tokens.eat(Token::OpAssign);
                let initializer = Initializer::parse(tokens);
                InitDeclarator::DeclaratorWithValue(Box::new(declarator), Box::new(initializer))
            }
            _ => InitDeclarator::Declarator(Box::new(declarator)),
        }
    }
}

impl Declarator {
    pub fn parse<'a>(tokens: &mut Tokens<'a>) -> Declarator {
        unimplemented!();
    }
}

impl Initializer {
    pub fn parse<'a>(tokens: &mut Tokens<'a>) -> Initializer {
        unimplemented!();
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
