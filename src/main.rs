use std::env;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;

fn main() -> Result<(), Box<dyn Error>> {
    let file_name = env::args().nth(1).expect("no file name supplied.");

    let mut source = String::new();
    File::open(&*file_name)?.read_to_string(&mut source)?;

    let num: i32 = source.trim().parse()?;

    println!("define i32 @main() #0 {{");
    println!("  ret i32 {}", num);
    println!("}}");

    Ok(())
}
