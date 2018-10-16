use super::code_gen_state::{CodeGenState, Variable};
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
                while Decl::lookahead_is_parsable(tokens) {
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

    pub fn gen_code(self, state: &mut CodeGenState) {
        for decl in self.decls {
            decl.gen_code(state);
        }

        for stmt in self.stmts {
            stmt.gen_code(state);
        }
    }
}

impl Stmt {
    pub fn parse<'a>(tokens: &mut Tokens<'a>) -> Stmt {
        match tokens.peek() {
            Some(Token::SyLBrace) => Stmt::Compound(Box::new(Compound::parse(tokens))),
            _ => Stmt::Jump(Box::new(Jump::parse(tokens))),
        }
    }

    pub fn gen_code(self, state: &mut CodeGenState) {
        match self {
            Stmt::Compound(compound) => compound.gen_code(state),
            Stmt::Jump(jump) => jump.gen_code(state),
        }
    }
}

impl Decl {
    pub fn lookahead_is_parsable<'a>(tokens: &Tokens<'a>) -> bool {
        DeclSpecifier::lookahead_is_parsable(tokens)
    }

    pub fn parse<'a>(tokens: &mut Tokens<'a>) -> Decl {
        let mut specs = Vec::new();
        while DeclSpecifier::lookahead_is_parsable(tokens) {
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

    pub fn gen_code(self, state: &mut CodeGenState) {
        assert_eq!(
            self.specs.len(),
            1,
            "currently only one type-specifier is supported for decleration."
        );

        let (tyir, align) = match self.specs.into_iter().next().unwrap() {
            DeclSpecifier::TypeSpecifier(tyspec) => match *tyspec {
                TypeSpecifier::Int => ("i32", 4),
            },
        };

        for init in self.inits {
            let (ident, init_value) = match init {
                InitDeclarator::Declarator(declarator) => match *declarator {
                    Declarator::Identifier(ident) => (ident, None),
                },
                InitDeclarator::DeclaratorWithValue(declarator, initializer) => {
                    match (*declarator, *initializer) {
                        (Declarator::Identifier(ident), Initializer::Additive(additive)) => {
                            (ident, Some(additive))
                        }
                    }
                }
            };

            if state.vars.contains_key(&ident) {
                panic!("variable {} is already defined.", ident);
            }

            let reg = state.next_reg();
            println!("  %{} = alloca {}, align {}", reg, tyir, align);
            state.vars.insert(ident, Variable::new(tyir, align, reg));
            if let Some(additive) = init_value {
                let res = additive.gen_code(state);

                println!(
                    "  store {} %{}, {}* %{}, align {}",
                    tyir, res, tyir, reg, align
                );
            }
        }
    }
}

impl DeclSpecifier {
    pub fn lookahead_is_parsable<'a>(tokens: &Tokens<'a>) -> bool {
        TypeSpecifier::lookahead_is_parsable(tokens)
    }

    pub fn parse<'a>(tokens: &mut Tokens<'a>) -> DeclSpecifier {
        DeclSpecifier::TypeSpecifier(Box::new(TypeSpecifier::parse(tokens)))
    }
}

impl TypeSpecifier {
    pub fn lookahead_is_parsable<'a>(tokens: &Tokens<'a>) -> bool {
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
        match tokens.next() {
            Some(Token::Ident(ident)) => Declarator::Identifier(ident.into()),
            other => panic!("expected identifier, found {:?}", other),
        }
    }
}

impl Initializer {
    pub fn parse<'a>(tokens: &mut Tokens<'a>) -> Initializer {
        Initializer::Additive(Box::new(AdditiveExpr::parse(tokens)))
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

    pub fn gen_code(self, state: &mut CodeGenState) {
        match self {
            Jump::Return(expr) => {
                let reg = expr.gen_code(state);
                println!("  ret i32 %{}", reg);

                // because instruction `ret` is terminator, llvm introduces unnamed block after
                // this instruction. this unnamed block is named by serial number, sharing the same
                // numbering as instruction. so we must skip that number for the further
                // (unreachable) instructions.
                state.next_reg();
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
