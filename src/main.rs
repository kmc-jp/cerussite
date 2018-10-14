extern crate itertools;

use std::env;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;

mod lexer;

use lexer::Lexer;

fn main() -> Result<(), Box<dyn Error>> {
    let file_name = env::args().nth(1).expect("no file name supplied.");

    let mut source = String::new();
    File::open(&*file_name)?.read_to_string(&mut source)?;

    let tokens: Vec<&str> = Lexer::from_source(&source).collect();

    eprintln!("{:?}", tokens);

    assert!(tokens.len() >= 9);

    let tokens = match (&tokens[0..7], &tokens[tokens.len() - 2..]) {
        (&["int", "main", "(", "void", ")", "{", "return"], &[";", "}"]) => {
            &tokens[7..tokens.len() - 2]
        }
        _ => panic!("compilation error"),
    };

    println!("define i32 @main() #0 {{");
    generate_code(tokens)?;
    println!("}}");

    Ok(())
}

fn generate_code(tokens: &[&str]) -> Result<(), Box<dyn Error>> {
    match tokens.len() {
        0 => panic!("compilation error"),
        1 => println!("  ret i32 {}", tokens[0].parse::<i32>()?),
        3 => handle_operator(tokens)?,
        _ => panic!("compilation error"),
    }

    Ok(())
}

fn handle_operator(tokens: &[&str]) -> Result<(), Box<dyn Error>> {
    assert_eq!(tokens.len(), 3);
    let op_mn = match tokens[1] {
        "+" => "add",
        "-" => "sub",
        "*" => "mul",
        "/" => "sdiv",
        op => panic!("operator '{}' is not yet supported.", op),
    };

    println!("  %1 = add i32 {}, 0", tokens[0].parse::<i32>()?);
    println!("  %2 = add i32 {}, 0", tokens[2].parse::<i32>()?);
    println!("  %3 = {} i32 %1, %2", op_mn);
    println!("  ret i32 %3");

    Ok(())
}
