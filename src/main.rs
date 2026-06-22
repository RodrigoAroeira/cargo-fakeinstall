use std::ffi::OsStr;
use std::io::Write;
use std::path::Path;

use anyhow::Result;
use clap::Parser;

mod cli;

fn main() -> Result<()> {
    let cli::Cli::Fakeinstall(args) = cli::Cli::parse();
    let url = &args.url;
    let bin_name = &args.bin_name;

    let temp = tempdir::TempDir::new("fakeinstall")?;
    let path = temp.path();
    cargo(["init", "--name", bin_name], Some(path));
    let mut file = std::fs::File::create(path.join("src").join("main.rs"))?;
    writeln!(file, r#"const URL: &str = "{}";"#, url)?;
    writeln!(file, r#"const BIN_NAME: &str = "{}";"#, bin_name)?;
    writeln!(file, "{S}")?;
    drop(file);
    cargo(
        ["install", "--path", path.display().to_string().as_str()],
        None,
    );
    println!("Run {bin_name} to boostrap your binary");

    Ok(())
}

const S: &str = r#"
use std::error::Error;
use std::os::unix::fs::PermissionsExt;
fn main() -> Result<(), Box<dyn Error>> {
    let path = std::env::current_exe()?;
    let cmd = std::process::Command::new("wget")
        .args(["-qO", BIN_NAME, URL])
        .spawn()
        .and_then(|mut c| c.wait())?;

    if !cmd.success() {
        Err(format!("Unable to download `{}` binary", BIN_NAME))?;
    }

    std::fs::set_permissions(BIN_NAME, std::fs::Permissions::from_mode(0o755))?;
    std::fs::rename(BIN_NAME, &path)?;
    println!("Successfully installed {BIN_NAME}!");

    Ok(())
}"#;

fn cargo<I, S>(args: I, temp: Option<&Path>)
where
    I: IntoIterator<Item = S>,
    S: AsRef<OsStr>,
{
    let mut cmd = std::process::Command::new("cargo");
    if let Some(dir) = temp {
        cmd.current_dir(dir);
    }
    cmd.args(args).spawn().and_then(|mut c| c.wait()).unwrap();
}
