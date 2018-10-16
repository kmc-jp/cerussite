/*
 * BNF which statement parser can accept for now
 *
 * <compound> ::=  { {<decl>}* {<stmt>}+ }
 *
 * <stmt> ::= <compound>
 *          | <jump>
 *
 * <decl> ::= {<decl-specifier>}+ {<init-declarator>}*};
 *
 * <decl-specifier> ::= <type-specifier>
 *
 * <type-specifier> ::= int
 *
 * <init-declarator> ::= <declarator>
 *                     | <declarator> = <initializer>
 *
 * <declarator> ::= <identifier>
 *
 * <initializer> ::= <additive-expr>
 *
 * <jump> ::= return <expr>;
 */
use super::code_gen_state::{CodeGenState, Variable};
use super::expr::{Additive as AdditiveExpr, Expr};
use super::{ParseError, Result};
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
    tyspec: Box<TypeSpecifier>,
    init: Box<InitDeclarator>,
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
    pub fn parse<'a>(tokens: &mut Tokens<'a>) -> Result<Compound> {
        match tokens.next() {
            Some(Token::SyLBrace) => {
                let mut decls = Vec::new();
                while let Some(Token::TyInt) = tokens.peek() {
                    decls.push(Decl::parse(tokens)?);
                }

                let mut stmts = Vec::new();
                loop {
                    stmts.push(Stmt::parse(tokens)?);
                    if let Some(Token::SyRBrace) = tokens.peek() {
                        tokens.eat(Token::SyRBrace);
                        break;
                    }
                }
                Ok(Compound { stmts, decls })
            }
            other => Err(ParseError::Unexpected {
                expected: "compound statement".into(),
                found: format!("{:?}", other),
            }),
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
    pub fn parse<'a>(tokens: &mut Tokens<'a>) -> Result<Stmt> {
        match tokens.peek() {
            Some(Token::SyLBrace) => Ok(Stmt::Compound(Box::new(Compound::parse(tokens)?))),
            _ => Ok(Stmt::Jump(Box::new(Jump::parse(tokens)?))),
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
    pub fn parse<'a>(tokens: &mut Tokens<'a>) -> Result<Decl> {
        let tyspec = Box::new(TypeSpecifier::parse(tokens)?);
        let init = Box::new(InitDeclarator::parse(tokens)?);
        tokens.eat(Token::SySemicolon);
        Ok(Decl { tyspec, init })
    }

    pub fn gen_code(self, state: &mut CodeGenState) {
        let (tyir, align) = match *self.tyspec {
            TypeSpecifier::Int => ("i32", 4),
        };

        // the below line is needed because of rust's limitation.
        // (rust can't handle destructing and unboxing box of tuple at the same time)
        let init = *self.init;
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

impl TypeSpecifier {
    pub fn parse<'a>(tokens: &mut Tokens<'a>) -> Result<TypeSpecifier> {
        match tokens.next() {
            Some(Token::TyInt) => Ok(TypeSpecifier::Int),
            other => Err(ParseError::Unexpected {
                expected: "type-specifier".into(),
                found: format!("{:?}", other),
            }),
        }
    }
}

impl InitDeclarator {
    pub fn parse<'a>(tokens: &mut Tokens<'a>) -> Result<InitDeclarator> {
        let declarator = Declarator::parse(tokens)?;
        match tokens.peek() {
            Some(Token::OpAssign) => {
                tokens.eat(Token::OpAssign);
                let initializer = Initializer::parse(tokens)?;
                Ok(InitDeclarator::DeclaratorWithValue(
                    Box::new(declarator),
                    Box::new(initializer),
                ))
            }
            _ => Ok(InitDeclarator::Declarator(Box::new(declarator))),
        }
    }
}

impl Declarator {
    pub fn parse<'a>(tokens: &mut Tokens<'a>) -> Result<Declarator> {
        match tokens.next() {
            Some(Token::Ident(ident)) => Ok(Declarator::Identifier(ident.into())),
            other => Err(ParseError::Unexpected {
                expected: "identifier".into(),
                found: format!("{:?}", other),
            }),
        }
    }
}

impl Initializer {
    pub fn parse<'a>(tokens: &mut Tokens<'a>) -> Result<Initializer> {
        Ok(Initializer::Additive(Box::new(AdditiveExpr::parse(
            tokens,
        )?)))
    }
}

impl Jump {
    pub fn parse<'a>(tokens: &mut Tokens<'a>) -> Result<Jump> {
        match tokens.next() {
            Some(Token::KwReturn) => {
                let expr = Expr::parse(tokens)?;
                tokens.eat_err(
                    Token::SySemicolon,
                    "missing semicolon after jump statement.",
                );
                Ok(Jump::Return(Box::new(expr)))
            }
            other => Err(ParseError::Unexpected {
                expected: "jump statement".into(),
                found: format!("{:?}", other),
            }),
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
