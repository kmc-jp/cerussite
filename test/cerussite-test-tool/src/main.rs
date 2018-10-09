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

/// returns the result of compilation with clang (for reference)
fn reference_compile(src_path: &Path) -> io::Result<(PathBuf, String)> {
    let output_path = {
        let output_name = prefixed_file_name(&src_path, "ref_");
        src_path.with_file_name(output_name).with_extension("ll")
    };

    // compile
    let output = Command::new("clang")
        .arg("-O0")
        .arg("-S")
        .arg("-emit-llvm")
        .arg("-o")
        .arg(output_path.display().to_string())
        .arg(src_path.display().to_string())
        .output()?;

    if !output.stderr.is_empty() {
        print_stderr(String::from_utf8_lossy(&output.stderr));
    }

    if !output_path.exists() {
        return Ok((output_path, "(compilation failed)".into()));
    }

    let mut output_file_string = String::new();
    File::open(&output_path)?.read_to_string(&mut output_file_string)?;

    Ok((output_path, output_file_string))
}

/// returns the result of compilation with our current compiler
fn current_compile(src_path: &Path) -> io::Result<(PathBuf, String)> {
    let output_path = {
        let output_name = prefixed_file_name(&src_path, "cur_");
        src_path.with_file_name(output_name).with_extension("ll")
    };

    // compile
    let output = Command::new("cargo")
        .arg("run")
        .arg("--")
        .arg(src_path.display().to_string())
        .output()?;

    if !output.stderr.is_empty() {
        print_stderr(String::from_utf8_lossy(&output.stderr));
    }

    File::create(&output_path)?.write_all(&output.stdout)?;
    let output_file_string = String::from_utf8_lossy(&output.stdout).into_owned();

    Ok((output_path, output_file_string))
}

fn compile_llvm_ir(src_path: &Path) -> io::Result<PathBuf> {
    let output_path = if cfg!(windows) {
        src_path.with_extension("exe")
    } else {
        src_path.with_file_name(
            src_path
                .file_stem()
                .expect("internal error: no file has no basename"),
        )
    };

    if src_path.exists() {
        let output = Command::new("clang")
            .arg("-o")
            .arg(&output_path)
            .arg(&src_path)
            .output()?;

        if !output.stderr.is_empty() {
            print_stderr(String::from_utf8_lossy(&output.stderr));
        }
    }

    Ok(output_path)
}

/// returns the execution of the binary placed in the specified path
fn execute(path: &Path) -> io::Result<(Option<i32>, String)> {
    if !path.exists() {
        return Ok((None, "(executable not found)".into()));
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

    if !stderr.is_empty() {
        print_stderr(stderr);
    }

    Ok((status.code(), stdout))
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
        colored_println!{
            true;
            Reset, "Removing {}", path.display();
        }
        fs::remove_file(&path)
    })?;

    walk_dir(&test_src_dir, false, |entry| {
        colored_println!{
            true;
            Reset, "File: ";
            Yellow, "{}\n", entry.path().display();
        }

        print_heading(LightGreen, "===>", "Reference");

        print_heading(Cyan, "->", "Compilation (C)");
        let (ref_llvm_ir, ref_compile) = reference_compile(&entry.path())?;
        print_output(None, &ref_compile);

        print_heading(Cyan, "->", "Compilation (LLVM IR)");
        let ref_execfile = compile_llvm_ir(&ref_llvm_ir)?;

        print_heading(Cyan, "->", "Execution");
        let (ref_retval, ref_output) = execute(&ref_execfile)?;
        print_output(ref_retval, &ref_output);

        // -------------------------------------------------------------------------------

        print_heading(LightGreen, "===>", "Current");

        print_heading(Cyan, "->", "Compilation (C)");
        let (cur_llvm_ir, cur_compile) = current_compile(&entry.path())?;
        print_output(None, &cur_compile);

        print_heading(Cyan, "->", "Compilation (LLVM IR)");
        let cur_execfile = compile_llvm_ir(&cur_llvm_ir)?;

        print_heading(Cyan, "->", "Execution");
        let (cur_retval, cur_output) = execute(&cur_execfile)?;
        print_output(cur_retval, &cur_output);

        let (color, judge) = if (ref_retval, ref_output) == (cur_retval, cur_output) {
            (Green, "OK")
        } else {
            (Red, "NG")
        };

        colored_println!{
            true;
            LightGreen, "===> ";
            Reset, "Result Matches? ";
            color, "{}\n", judge;
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
