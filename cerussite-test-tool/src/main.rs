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

enum CompilationResult {
    Success {
        ir_path: PathBuf,
        llvm_ir: String,
        cc_output: String,
    },
    Failure {
        cc_output: String,
    },
}

/// returns the result of compilation with clang (for reference)
fn reference_compile(src_path: &Path) -> io::Result<CompilationResult> {
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
        return Ok(CompilationResult::Failure { cc_output });
    }

    let mut llvm_ir = String::new();
    File::open(&ir_path)?.read_to_string(&mut llvm_ir)?;

    Ok(CompilationResult::Success {
        ir_path,
        llvm_ir,
        cc_output,
    })
}

/// returns the llvm_ir of compilation with our current compiler
fn current_compile(src_path: &Path) -> io::Result<CompilationResult> {
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

    if output.stdout.is_empty() {
        // compilation failed.
        return Ok(CompilationResult::Failure { cc_output });
    }

    File::create(&ir_path)?.write_all(&output.stdout)?;
    let llvm_ir = String::from_utf8_lossy(&output.stdout).into_owned();

    Ok(CompilationResult::Success {
        ir_path,
        llvm_ir,
        cc_output,
    })
}

enum AssemblyResult {
    Success {
        asm_output: String,
        exec_path: PathBuf,
    },
    Failure {
        asm_output: String,
    },
    Unreached,
}

fn compile_llvm_ir(src_path: &Path) -> io::Result<AssemblyResult> {
    let exec_path = if cfg!(windows) {
        src_path.with_extension("exe")
    } else {
        let file_name = src_path
            .file_stem()
            .expect("internal error: no file has no basename");
        src_path.with_file_name(file_name)
    };

    if !src_path.exists() {
        panic!("internal error: compilation has succeeded but no LLVM IR?");
    }

    let output = Command::new("clang")
        .arg("-o")
        .arg(&exec_path)
        .arg(&src_path)
        .output()?;

    let asm_output = String::from_utf8_lossy(&output.stderr).into_owned();

    if !exec_path.exists() {
        return Ok(AssemblyResult::Failure { asm_output });
    }

    Ok(AssemblyResult::Success {
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
    Unreached,
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
    pub fn get_compiler_func(&self) -> fn(path: &Path) -> io::Result<CompilationResult> {
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
    compilation: CompilationResult,
    assembly: AssemblyResult,
    execution: ExecutionResult,
}

fn do_for(version: Version, path: &Path) -> io::Result<Results> {
    let (compilation, assembly, execution);

    // explicitly denote borrowing region
    {
        compilation = (version.get_compiler_func())(&path)?;
        let ir_path = match compilation {
            failure @ CompilationResult::Failure { .. } => {
                return Ok(Results {
                    compilation: failure,
                    assembly: AssemblyResult::Unreached,
                    execution: ExecutionResult::Unreached,
                })
            }
            CompilationResult::Success { ref ir_path, .. } => ir_path.clone(),
        };

        assembly = compile_llvm_ir(&ir_path)?;
        let exec_path = match assembly {
            failure @ AssemblyResult::Failure { .. } => {
                return Ok(Results {
                    compilation: compilation,
                    assembly: failure,
                    execution: ExecutionResult::Unreached,
                })
            }
            AssemblyResult::Success { ref exec_path, .. } => exec_path.clone(),
            AssemblyResult::Unreached => unreachable!(),
        };

        execution = execute(&exec_path)?;
    }

    Ok(Results {
        compilation,
        assembly,
        execution,
    })
}

fn judge(refr: &ExecutionResult, curr: &ExecutionResult) -> (bool, ConsoleColor, &'static str) {
    const OK: (bool, ConsoleColor, &str) = (true, Green, "OK");
    const NG: (bool, ConsoleColor, &str) = (false, Red, "NG");

    use ExecutionResult::Success;

    match (refr, curr) {
        (
            Success {
                status: ref refr_status,
                stdout: ref refr_stdout,
                ..
            },
            Success {
                status: ref curr_status,
                stdout: ref curr_stdout,
                ..
            },
        ) => {
            if (refr_status, refr_stdout) == (curr_status, curr_stdout) {
                OK
            } else {
                NG
            }
        }
        _ => NG,
    }
}

fn print_for(version: Version, results: Results) {
    print_heading(
        LightGreen,
        &format!("==================== {} ====================", version),
    );

    use {AssemblyResult as AR, CompilationResult as CR, ExecutionResult as ER};

    print_heading(LightBlue, "> Compilation (C)");
    match results.compilation {
        CR::Success {
            cc_output, llvm_ir, ..
        } => {
            print_stderr(&cc_output);
            print_output(None, &llvm_ir);
        }
        CR::Failure { cc_output, .. } => {
            print_stderr(&cc_output);
            return;
        }
    }

    print_heading(LightBlue, "> Compilation (LLVM IR)");
    match results.assembly {
        AR::Success { asm_output, .. } => {
            print_stderr(&asm_output);
        }
        AR::Failure { asm_output, .. } => {
            print_stderr(&asm_output);
            return;
        }
        AR::Unreached => unreachable!(),
    }

    print_heading(LightBlue, "> Execution");
    match results.execution {
        ER::Success {
            status,
            stdout,
            stderr,
        } => {
            print_stderr(&stderr);
            print_output(status, &stdout);
        }
        ER::Unreached => unreachable!(),
    }
}

fn main() -> io::Result<()> {
    let verbose = env::args().any(|arg| arg == "--verbose" || arg == "-v");
    let test_src_dir: PathBuf = ["test", "ok"].iter().collect();

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

        let (status, color, judge) = judge(&refr.execution, &curr.execution);

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
