use std::ffi::OsStr;
use std::io::Write;
use std::path::Path;

use anyhow::{Context, Result};
use clap::Parser;

mod cli;
mod uri;

use uri::Uri;

fn main() -> Result<()> {
    let cli::Cli::Fakeinstall(args) = cli::Cli::parse();
    let uri = &args.uri;
    let bin_name = &args.bin_name;

    let temp = tempdir::TempDir::new("fakeinstall")?;
    let path = temp.path();
    cargo(["init", "--name", bin_name], Some(path)).context("failed to create cargo project")?;
    {
        let mut file = std::fs::File::create(path.join("src").join("main.rs"))?;
        writeln!(file, "{}", bootstrap_source(uri, bin_name))?;
    }
    cargo(["install", "--path", "."], Some(path)).context("failed to install bootstrapper")?;
    println!("Run {bin_name} to boostrap your binary");

    Ok(())
}

fn bootstrap_source(uri: &Uri, bin_name: &str) -> String {
    match uri {
        Uri::Remote(url) => {
            println!("Found remote url `{url}`");
            format!(
                r#"
use std::os::unix::fs::PermissionsExt;
fn main() -> Result<(), Box<dyn std::error::Error>> {{
    let path = std::env::current_exe()?;
    let tmp = path.with_extension("tmp");
    let cmd = std::process::Command::new("wget")
        .args(["-qO", &tmp, "{url}"])
        .spawn()
        .and_then(|mut c| c.wait())?;
    if !cmd.success() {{
        Err(format!("Unable to download `{{}}` binary", "{bin_name}"))?;
    }}
    std::fs::set_permissions(&tmp, std::fs::Permissions::from_mode(0o755))?;
    std::fs::rename(&tmp, &path)?;
    println!("Successfully installed {bin_name}!");
    Ok(())
}}"#,
            )
        }
        Uri::Local(source) => {
            println!("Found local file `{source}`");
            format!(
                r#"
use std::os::unix::fs::PermissionsExt;
fn main() -> Result<(), Box<dyn std::error::Error>> {{
    let path = std::env::current_exe()?;
    let tmp = path.with_extension("tmp");
    std::fs::copy("{source}", &tmp)?;
    std::fs::set_permissions(&tmp, std::fs::Permissions::from_mode(0o755))?;
    std::fs::rename(&tmp, &path)?;
    println!("Successfully installed {bin_name}!");
    Ok(())
}}"#,
            )
        }
    }
}

fn cargo<I, S>(args: I, temp: Option<&Path>) -> Result<()>
where
    I: IntoIterator<Item = S>,
    S: AsRef<OsStr>,
{
    let mut cmd = std::process::Command::new("cargo");
    if let Some(dir) = temp {
        cmd.current_dir(dir);
    }
    cmd.args(args).spawn().and_then(|mut c| c.wait())?;
    Ok(())
}
