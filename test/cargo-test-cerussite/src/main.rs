use std::env;
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

    let mut args = env::args();

    assert_eq!(args.next().map(|x| x.contains("cargo")), Some(true));
    assert_eq!(args.next(), Some("test-cerussite".into()));

    // run test tool
    Command::new("test/cerussite-test-tool/target/release/cerussite-test-tool")
        .args(args)
        .spawn()?
        .wait()?;

    Ok(())
}
