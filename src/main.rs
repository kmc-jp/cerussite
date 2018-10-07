use std::env;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;

fn main() -> Result<(), Box<dyn Error>> {
    let file_name = env::args().nth(1).expect("no file name supplied.");

    let mut source = String::new();
    File::open(&*file_name)?.read_to_string(&mut source)?;

    let operands: Vec<&str> = source.split("+").collect();
    let mut res: i32 = 0;
    // Check if the source file consists of only an empty expression.
    // if so, generate LLVM IR that does nothing (returns 0).
    if operands.len() == 1 && operands[0].trim().is_empty() {
        res = 0;
    } else {
        let mut operands_parsed: Vec<i32> = Vec::new();
        for o in operands {
            let o = o.trim();
            operands_parsed.push(o.parse()?);
        }
        for op in operands_parsed {
            res += op;
        }
    }

    println!("define i32 @main() #0 {{");
    println!("  ret i32 {}", res);
    println!("}}");

    Ok(())
}
