#[macro_use]
extern crate colored_print;

use colored_print::color::ConsoleColor;
use colored_print::color::ConsoleColor::*;

use std::fmt::Display;
use std::fs;
use std::fs::{DirEntry, File};
use std::io;
use std::io::prelude::*;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

fn prefixed_file_name(path: &Path, prefix: &str) -> String {
    let name = path
        .file_name()
        .expect("internal error: there are no files without name!")
        .to_str()
        .expect("internal error: file name cannot be represented in UTF-8.");
    format!("{}{}", prefix, name)
}

struct CompilerResult {
    ir_path: PathBuf,
    llvm_ir: String,
    cc_output: String,
}

/// returns the result of compilation with clang (for reference)
fn reference_compile(src_path: &Path) -> io::Result<CompilerResult> {
    let ir_path = {
        let output_name = prefixed_file_name(&src_path, "ref_");
        src_path.with_file_name(output_name).with_extension("ll")
    };

    // compile
    let output = Command::new("clang")
        .arg("-O0")
        .arg("-S")
        .arg("-emit-llvm")
        .arg("-o")
        .arg(ir_path.display().to_string())
        .arg(src_path.display().to_string())
        .output()?;

    let cc_output = String::from_utf8_lossy(&output.stderr).into_owned();

    if !ir_path.exists() {
        return Ok(CompilerResult {
            ir_path,
            cc_output,
            llvm_ir: String::new(),
        });
    }

    let mut llvm_ir = String::new();
    File::open(&ir_path)?.read_to_string(&mut llvm_ir)?;

    Ok(CompilerResult {
        ir_path,
        llvm_ir,
        cc_output,
    })
}

/// returns the llvm_ir of compilation with our current compiler
fn current_compile(src_path: &Path) -> io::Result<CompilerResult> {
    let ir_path = {
        let output_name = prefixed_file_name(&src_path, "cur_");
        src_path.with_file_name(output_name).with_extension("ll")
    };

    // compile
    let output = Command::new("cargo")
        .arg("run")
        .arg("--")
        .arg(src_path.display().to_string())
        .output()?;

    let cc_output = String::from_utf8_lossy(&output.stderr).into_owned();
    File::create(&ir_path)?.write_all(&output.stdout)?;
    let llvm_ir = String::from_utf8_lossy(&output.stdout).into_owned();

    Ok(CompilerResult {
        ir_path,
        llvm_ir,
        cc_output,
    })
}

struct AssemblerResult {
    asm_output: String,
    exec_path: PathBuf,
}

fn compile_llvm_ir(src_path: &Path) -> io::Result<AssemblerResult> {
    let exec_path = if cfg!(windows) {
        src_path.with_extension("exe")
    } else {
        let file_name = src_path
            .file_stem()
            .expect("internal error: no file has no basename");
        src_path.with_file_name(file_name)
    };

    if !src_path.exists() {
        return Ok(AssemblerResult {
            exec_path,
            asm_output: String::new(),
        });
    }

    let output = Command::new("clang")
        .arg("-o")
        .arg(&exec_path)
        .arg(&src_path)
        .output()?;

    let asm_output = String::from_utf8_lossy(&output.stderr).into_owned();

    Ok(AssemblerResult {
        asm_output,
        exec_path,
    })
}

struct ExecutionResult {
    status: Option<i32>,
    stdout: String,
    stderr: String,
}

/// returns the execution of the binary placed in the specified path
fn execute(path: &Path) -> io::Result<ExecutionResult> {
    if !path.exists() {
        return Ok(ExecutionResult {
            status: None,
            stdout: String::new(),
            stderr: String::new(),
        });
    }

    let mut child = Command::new(&path)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()?;

    let status = child.wait()?;

    let mut child_stdout = child
        .stdout
        .expect("internal error: failed to get child stdout.");

    let mut child_stderr = child
        .stderr
        .expect("internel error: failed to get child stderr.");

    let (mut stdout, mut stderr) = (String::new(), String::new());
    child_stdout.read_to_string(&mut stdout)?;
    child_stderr.read_to_string(&mut stderr)?;
    let status = status.code();

    Ok(ExecutionResult {
        status,
        stdout,
        stderr,
    })
}

fn print_stderr(stderr: impl Display) {
    // indent output
    let stderr = stderr
        .to_string()
        .split('\n')
        .map(|x| format!("    {}", x))
        .collect::<Vec<_>>()
        .join("\n");
    colored_println!{
        true;
        LightMagenta, "{}\n", stderr;
    }
}

fn print_heading(color: ConsoleColor, symbol: &str, heading: &str) {
    colored_println!{
        true;
        color, "{} ", symbol;
        Reset, "{}\n", heading;
    }
}
fn print_output(retval: Option<i32>, output: &str) {
    colored_println!{
        true;
        Reset, "{}\n", output;
    }
    if let Some(code) = retval {
        colored_println!{
            true;
            LightBlue, "return code";
            Reset, ": {}\n", code;
        }
    }
}

fn main() -> io::Result<()> {
    let test_src_dir: PathBuf = ["test", "test-src", "ok"].iter().collect();

    walk_dir(&test_src_dir, true, |entry| {
        let path = entry.path();
        colored_println! {
            true;
            LightGreen, "Removing ";
            Reset, "{}", path.display();
        }
        fs::remove_file(&path)
    })?;

    walk_dir(&test_src_dir, false, |entry| {
        colored_print!{
            true;
            LightGreen, " Testing ";
            Reset, "file ";
            Yellow, "{}", entry.path().display();
            Reset, " ... ";
        }

        let ref_compile = reference_compile(&entry.path())?;
        let ref_assemble = compile_llvm_ir(&ref_compile.ir_path)?;
        let ref_execute = execute(&ref_assemble.exec_path)?;

        let cur_compile = current_compile(&entry.path())?;
        let cur_assemble = compile_llvm_ir(&cur_compile.ir_path)?;
        let cur_execute = execute(&cur_assemble.exec_path)?;

        let ref_res = (&ref_execute.status, &ref_execute.stdout);
        let cur_res = (&cur_execute.status, &cur_execute.stdout);
        let status = ref_res == cur_res;
        let (color, judge) = if status { (Green, "OK") } else { (Red, "NG") };

        colored_println!{
            true;
            color, "{}", judge;
        }

        // print info only when failure
        if !status {
            print_heading(LightGreen, "===>", "Reference");

            print_heading(Cyan, "->", "Compilation (C)");
            if !ref_compile.cc_output.is_empty() {
                print_stderr(&ref_compile.cc_output);
            }
            print_output(None, &ref_compile.llvm_ir);

            print_heading(Cyan, "->", "Compilation (LLVM IR)");
            if !ref_assemble.asm_output.is_empty() {
                print_stderr(&ref_assemble.asm_output);
            }

            print_heading(Cyan, "->", "Execution");
            if !ref_execute.stderr.is_empty() {
                print_stderr(&ref_execute.stderr);
            }
            print_output(ref_execute.status, &ref_execute.stdout);

            // -------------------------------------------------------------------------------

            print_heading(LightGreen, "===>", "Current");

            print_heading(Cyan, "->", "Compilation (C)");
            if !cur_compile.cc_output.is_empty() {
                print_stderr(&cur_compile.cc_output);
            }
            print_output(None, &cur_compile.llvm_ir);

            print_heading(Cyan, "->", "Compilation (LLVM IR)");
            if !cur_assemble.asm_output.is_empty() {
                print_stderr(&cur_assemble.asm_output);
            }

            print_heading(Cyan, "->", "Execution");
            if !cur_execute.stderr.is_empty() {
                print_stderr(&cur_execute.stderr);
            }
            print_output(cur_execute.status, &cur_execute.stdout);
        }

        Ok(())
    })
}

fn walk_dir(
    dir: &Path,
    cleanup_walking: bool,
    cb: impl Fn(&DirEntry) -> io::Result<()> + Copy,
) -> io::Result<()> {
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            walk_dir(&path, cleanup_walking, cb)?;
        } else {
            let ext = path
                .extension()
                .map(|x| {
                    x.to_str()
                        .expect("internal error: file name cannot be represented in UTF-8.")
                })
                .unwrap_or("");
            match (cleanup_walking, ext == "c") {
                (false, true) | (true, false) => {
                    cb(&entry)?;
                }
                _ => {}
            }
        }
    }

    Ok(())
}
