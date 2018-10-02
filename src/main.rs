use std::env;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;

fn tokenize(source: &str) -> Vec<&str> {
    // currently the supported delimiters are [' ', '\n'].
    source
        .split(' ')
        .flat_map(|x| x.split('\n'))
        .map(str::trim)
        .filter(|&x| x != "")
        .collect()
}

fn main() -> Result<(), Box<dyn Error>> {
    let file_name = env::args().nth(1).expect("no file name supplied.");

    let mut source = String::new();
    File::open(&*file_name)?.read_to_string(&mut source)?;

    let tokens = tokenize(&source);

    eprintln!("{:?}", tokens);
    let tokens = match (&tokens[0..7], &tokens[tokens.len() - 2..]) {
        (&["int", "main", "(", "void", ")", "{", "return"], &[";", "}"]) => {
            &tokens[7..tokens.len() - 2]
        }
        _ => panic!("compilation error"),
    };

    let num: i32 = tokens[0].trim().parse()?;

    println!("define i32 @main() #0 {{");
    println!("  ret i32 {}", num);
    println!("}}");

    Ok(())
}
