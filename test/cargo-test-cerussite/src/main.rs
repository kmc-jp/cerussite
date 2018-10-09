use std::io;
use std::process::Command;

fn main() -> io::Result<()> {
    // build test tool
    Command::new("cargo")
        .arg("build")
        .arg("--release")
        .current_dir("test/cerussite-test-tool")
        .spawn()?
        .wait()?;

    Ok(())
}
