extern crate itertools;
#[macro_use]
extern crate lazy_static;
extern crate regex;

use std::env;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;

mod ast;
mod lexer;
mod token;

use ast::Ast;
use lexer::Lexer;
use token::Token;

fn main() -> Result<(), Box<dyn Error>> {
    let file_name = env::args().nth(1).expect("no file name supplied.");

    let mut source = String::new();
    File::open(&*file_name)?.read_to_string(&mut source)?;

    let tokens: Vec<Token> = Lexer::from_source(&source).collect();

    eprintln!("{:?}", tokens);

    assert!(tokens.len() >= 9);

    let tokens = match (&tokens[0..7], &tokens[tokens.len() - 2..]) {
        (
            &[Token::TyInt, Token::Ident("main"), Token::SyLPar, Token::TyVoid, Token::SyRPar, Token::SyLBrace, Token::KwReturn],
            &[Token::SySemicolon, Token::SyRBrace],
        ) => &tokens[7..tokens.len() - 2],
        _ => panic!("compilation error"),
    };

    let ast = Ast::parse(tokens);

    println!("define i32 @main() #0 {{");
    let reg = ast.gen_code();
    println!("  ret i32 %{}", reg);
    println!("}}");

    Ok(())
}
