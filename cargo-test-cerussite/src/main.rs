use std::env;
use std::io;
use std::path::PathBuf;
use std::process::Command;

fn main() -> io::Result<()> {
    let root = find_project_root()?;
    eprintln!("> found project root at {}", root.display());

    // build test tool
    eprintln!("> building test tool");
    let res = Command::new("cargo")
        .arg("build")
        .arg("--release")
        .current_dir(root.join("cerussite-test-tool"))
        .spawn()?
        .wait()?;
    if !res.success() {
        return Err(io::Error::new(
            io::ErrorKind::Other,
            "building test tool failed.",
        ));
    }

    // build cerussite
    eprintln!("> building cerussite");
    let res = Command::new("cargo")
        .arg("build")
        .current_dir(&root)
        .spawn()?
        .wait()?;
    if !res.success() {
        return Err(io::Error::new(
            io::ErrorKind::Other,
            "building cerussite failed.",
        ));
    }

    // test cerussite
    eprintln!("> testing cerussite");
    let res = Command::new("cargo")
        .arg("test")
        .current_dir(&root)
        .spawn()?
        .wait()?;
    if !res.success() {
        return Err(io::Error::new(
            io::ErrorKind::Other,
            "testing cerussite failed.",
        ));
    }

    // run test tool
    eprintln!("> running test");
    let mut args = env::args();

    assert_eq!(args.next().map(|x| x.contains("cargo")), Some(true));
    assert_eq!(args.next(), Some("test-cerussite".into()));

    let res = Command::new(root.join("cerussite-test-tool/target/release/cerussite-test-tool"))
        .args(args)
        .current_dir(root)
        .spawn()?
        .wait()?;
    if !res.success() {
        return Err(io::Error::new(
            io::ErrorKind::Other,
            "runnning test failed.",
        ));
    }

    Ok(())
}

fn find_project_root() -> io::Result<PathBuf> {
    let mut may_root = env::current_dir()?;
    assert!(may_root.is_absolute());

    loop {
        if may_root.join("Cargo.toml").exists() {
            return Ok(may_root);
        }
        if let Some(par) = may_root.parent().map(|x| x.to_path_buf()) {
            may_root = par;
        } else {
            break;
        }
    }

    Err(io::Error::new(
        io::ErrorKind::Other,
        "no project root detected (ran into filesystem root)",
    ))
}
