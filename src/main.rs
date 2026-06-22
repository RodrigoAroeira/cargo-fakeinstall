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
    {
        let mut file = std::fs::File::create(path.join("src").join("main.rs"))?;
        writeln!(file, "{}", bootstrap_source(url, bin_name))?;
    }
    cargo(["install", "--path", "."], Some(path));
    println!("Run {bin_name} to boostrap your binary");

    Ok(())
}

fn bootstrap_source(url: &str, bin_name: &str) -> String {
    format!(
        r#"
use std::error::Error;
use std::os::unix::fs::PermissionsExt;
fn main() -> Result<(), Box<dyn Error>> {{
    let path = std::env::current_exe()?;
    let cmd = std::process::Command::new("wget")
        .args(["-qO", "{bin_name}", "{url}"])
        .spawn()
        .and_then(|mut c| c.wait())?;

    if !cmd.success() {{
        Err(format!("Unable to download `{{}}` binary", "{bin_name}"))?;
    }}

    std::fs::set_permissions("{bin_name}", std::fs::Permissions::from_mode(0o755))?;
    std::fs::rename("{bin_name}", &path)?;
    println!("Successfully installed {bin_name}!");

    Ok(())
}}"#,
        url = url,
        bin_name = bin_name
    )
}

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
