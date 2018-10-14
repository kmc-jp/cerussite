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

    println!("define i32 @main() #0 {{");
    generate_code(tokens)?;
    println!("}}");

    Ok(())
}

fn generate_code(tokens: &[Token]) -> Result<(), Box<dyn Error>> {
    match tokens.len() {
        0 => panic!("compilation error"),
        1 => println!("  ret i32 {}", tokens[0].unwrap_literal()),
        3 => handle_operator(tokens)?,
        _ => panic!("compilation error"),
    }

    Ok(())
}

fn handle_operator(tokens: &[Token]) -> Result<(), Box<dyn Error>> {
    assert_eq!(tokens.len(), 3);
    let op_mn = match tokens[1] {
        Token::OpAdd => "add",
        Token::OpSub => "sub",
        Token::OpMul => "mul",
        Token::OpDiv => "sdiv",
        op => panic!("operator '{:?}' is not yet supported.", op),
    };

    println!(
        "  %1 = add i32 {}, 0",
        tokens[0].unwrap_literal().parse::<i32>()?
    );
    println!(
        "  %2 = add i32 {}, 0",
        tokens[2].unwrap_literal().parse::<i32>()?
    );
    println!("  %3 = {} i32 %1, %2", op_mn);
    println!("  ret i32 %3");

    Ok(())
}
