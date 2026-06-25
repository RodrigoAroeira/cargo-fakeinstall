use std::ffi::OsStr;
use std::io::Write;
use std::path::Path;

use anyhow::{Context, Result};
use clap::Parser;

mod cli;
mod uri;

use uri::Uri;

mod template {
    pub const REMOTE: &str = include_str!("../templates/remote.rs");
    pub const LOCAL: &str = include_str!("../templates/local.rs");
}

fn main() -> Result<()> {
    let cli::Cli::Fakeinstall(args) = cli::Cli::parse();
    let uri = &args.uri;
    let bin_name = &args.bin_name;

    let temp = tempdir::TempDir::new("fakeinstall")?;
    let path = temp.path();
    cargo(["init", "--name", bin_name], Some(path), args.verbose)
        .context("failed to create cargo project")?;
    {
        let mut file = std::fs::File::create(path.join("src").join("main.rs"))?;
        writeln!(file, "{}", bootstrap_source(uri, bin_name))?;
    }
    cargo(["install", "--path", "."], Some(path), args.verbose)
        .context("failed to install bootstrapper")?;
    println!("Run {bin_name} to boostrap your binary");

    Ok(())
}

fn bootstrap_source(uri: &Uri, bin_name: &str) -> String {
    let (template_str, uri) = match uri {
        Uri::Remote(url) => {
            println!("Found remote url `{url}`");
            (template::REMOTE, url)
        }
        Uri::Local(source) => {
            println!("Found local file `{source}`");
            (template::LOCAL, source)
        }
    };
    template_str
        .replace("@@BIN_NAME@@", bin_name)
        .replace("@@BIN_URI@@", uri)
}

fn cargo<I, S>(args: I, temp: Option<&Path>, verbose: u8) -> Result<()>
where
    I: IntoIterator<Item = S>,
    S: AsRef<OsStr>,
{
    let mut cmd = std::process::Command::new("cargo");

    if verbose < 2 {
        cmd.stderr(std::process::Stdio::null());
    }
    if verbose < 1 {
        cmd.stdout(std::process::Stdio::null());
    }

    if let Some(dir) = temp {
        cmd.current_dir(dir);
    }

    cmd.args(args).spawn().and_then(|mut c| c.wait())?;
    Ok(())
}
