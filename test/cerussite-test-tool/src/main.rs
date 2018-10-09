#[macro_use]
extern crate colored_print;
extern crate atty;

use colored_print::color::ConsoleColor;
use colored_print::color::ConsoleColor::*;

use std::env;
use std::ffi::OsStr;
use std::fmt;
use std::fmt::Display;
use std::fs;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

fn colorize() -> bool {
    use atty;
    use atty::Stream;

    atty::is(Stream::Stdout)
}

fn prefixed_file_name(path: &Path, prefix: &str) -> String {
    let name = path
        .file_name()
        .expect("internal error: there are no files without name!")
        .to_str()
        .expect("internal error: file name cannot be represented in UTF-8.");
    format!("{}{}", prefix, name)
}

enum CompilerResult {
    Success {
        ir_path: PathBuf,
        llvm_ir: String,
        cc_output: String,
    },
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
        return Ok(CompilerResult::Success {
            ir_path,
            cc_output,
            llvm_ir: String::new(),
        });
    }

    let mut llvm_ir = String::new();
    File::open(&ir_path)?.read_to_string(&mut llvm_ir)?;

    Ok(CompilerResult::Success {
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

    Ok(CompilerResult::Success {
        ir_path,
        llvm_ir,
        cc_output,
    })
}

enum AssemblerResult {
    Success {
        asm_output: String,
        exec_path: PathBuf,
    },
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
        return Ok(AssemblerResult::Success {
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

    Ok(AssemblerResult::Success {
        asm_output,
        exec_path,
    })
}

enum ExecutionResult {
    Success {
        status: Option<i32>,
        stdout: String,
        stderr: String,
    },
}

/// returns the execution of the binary placed in the specified path
fn execute(path: &Path) -> io::Result<ExecutionResult> {
    if !path.exists() {
        return Ok(ExecutionResult::Success {
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

    Ok(ExecutionResult::Success {
        status,
        stdout,
        stderr,
    })
}

fn print_heading(color: ConsoleColor, heading: &str) {
    colored_println!{
        colorize();
        color, "{} ", heading;
    }
}

fn print_output(retval: Option<i32>, output: &str) {
    colored_print!{
        colorize();
        Reset, "{}", output;
    }
    if let Some(code) = retval {
        colored_println!{
            colorize();
            Cyan, "return code";
            Reset, ": {}", code;
        }
    }
}

fn print_stderr(stderr: impl Display) {
    colored_print!{
        colorize();
        LightMagenta, "{}", stderr;
    }
}

#[derive(Debug, Copy, Clone)]
enum Version {
    Reference,
    Current,
}

impl Version {
    pub fn get_compiler_func(&self) -> fn(path: &Path) -> io::Result<CompilerResult> {
        match *self {
            Version::Reference => reference_compile,
            Version::Current => current_compile,
        }
    }
}

impl fmt::Display for Version {
    fn fmt(&self, b: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Version::Reference => write!(b, "Reference"),
            Version::Current => write!(b, " Current "),
        }
    }
}

struct Results {
    compile: CompilerResult,
    assemble: AssemblerResult,
    execute: ExecutionResult,
}

fn do_for(version: Version, path: &Path) -> io::Result<Results> {
    let (res_compile, res_assemble, res_execute);

    // explicitly denote borrowing region
    {
        res_compile = (version.get_compiler_func())(&path)?;
        let CompilerResult::Success { ref ir_path, .. } = res_compile;
        res_assemble = compile_llvm_ir(ir_path)?;
        let AssemblerResult::Success { ref exec_path, .. } = res_assemble;
        res_execute = execute(exec_path)?;
    }

    Ok(Results {
        compile: res_compile,
        assemble: res_assemble,
        execute: res_execute,
    })
}

fn judge(refr: &Results, curr: &Results) -> (bool, ConsoleColor, &'static str) {
    let ExecutionResult::Success {
        status: ref refr_status,
        stdout: ref refr_stdout,
        ..
    } = refr.execute;
    let ExecutionResult::Success {
        status: ref curr_status,
        stdout: ref curr_stdout,
        ..
    } = curr.execute;
    let refr_res = (refr_status, refr_stdout);
    let curr_res = (curr_status, curr_stdout);
    if refr_res == curr_res {
        (true, Green, "OK")
    } else {
        (false, Red, "NG")
    }
}

fn print_for(version: Version, results: Results) {
    print_heading(
        LightGreen,
        &format!("==================== {} ====================", version),
    );

    print_heading(LightBlue, "> Compilation (C)");
    let CompilerResult::Success {
        cc_output, llvm_ir, ..
    } = results.compile;
    if !cc_output.is_empty() {
        print_stderr(&cc_output);
    }
    print_output(None, &llvm_ir);

    print_heading(LightBlue, "> Compilation (LLVM IR)");
    let AssemblerResult::Success { asm_output, .. } = results.assemble;
    if !asm_output.is_empty() {
        print_stderr(&asm_output);
    }

    print_heading(LightBlue, "> Execution");
    let ExecutionResult::Success {
        status,
        stdout,
        stderr,
    } = results.execute;
    if !stderr.is_empty() {
        print_stderr(&stderr);
    }
    print_output(status, &stdout);
}

fn main() -> io::Result<()> {
    let verbose = env::args().any(|arg| arg == "--verbose" || arg == "-v");
    let test_src_dir: PathBuf = ["test", "test-src", "ok"].iter().collect();

    walk_dir(
        &test_src_dir,
        |path| path.extension().and_then(OsStr::to_str) != Some("c"),
        |path| {
            colored_println! {
                colorize();
                LightGreen, "Removing ";
                Reset, "{}", path.display();
            }
            fs::remove_file(&path)
        },
    )?;

    let mut path_to_test: Vec<_> = env::args()
        .skip(1)
        .filter(|arg| !arg.starts_with("-"))
        .map(|file_name| test_src_dir.join(file_name))
        .collect();

    if path_to_test.is_empty() {
        path_to_test = walk_dir(
            &test_src_dir,
            |path| path.extension().and_then(OsStr::to_str) == Some("c"),
            |path| Ok(path.to_path_buf()),
        )?;
    }

    for path in path_to_test {
        colored_print!{
            colorize();
            LightGreen, " Testing ";
            Reset, "file ";
            Yellow, "{}", path.display();
            Reset, " ... ";
        }

        if !path.exists() {
            println!("not found");
            continue;
        }

        let refr = do_for(Version::Reference, &path)?;
        let curr = do_for(Version::Current, &path)?;

        let (status, color, judge) = judge(&refr, &curr);

        colored_println!{
            colorize();
            color, "{}", judge;
        }

        // print info when verbose mode or something fails
        if verbose || !status {
            print_for(Version::Reference, refr);
            print_for(Version::Current, curr);
        }
    }

    Ok(())
}

fn walk_dir<T>(
    dir: &Path,
    path_filter: impl Fn(&Path) -> bool + Copy,
    cb: impl Fn(&Path) -> io::Result<T> + Copy,
) -> io::Result<Vec<T>> {
    let mut result = Vec::new();
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        if !path_filter(&path) {
            continue;
        }

        if path.is_dir() {
            walk_dir(&path, path_filter, cb)?;
        } else {
            result.push(cb(&path)?);
        }
    }

    Ok(result)
}
