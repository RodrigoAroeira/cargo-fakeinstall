use std::os::unix::fs::PermissionsExt;

const BIN_NAME: &str = "@@BIN_NAME@@";
const BIN_URI: &str = "@@BIN_URI@@";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let path = std::env::current_exe()?;
    let tmp = path.with_extension("tmp");
    let cmd = std::process::Command::new("wget")
        .args(["-qO", &tmp, BIN_URI])
        .spawn()
        .and_then(|mut c| c.wait())?;
    if !cmd.success() {
        Err(format!("Unable to download `{BIN_NAME}` binary"))?;
    }
    std::fs::set_permissions(&tmp, std::fs::Permissions::from_mode(0o755))?;
    std::fs::rename(&tmp, &path)?;
    println!("Successfully installed `{BIN_NAME}`!");
    Ok(())
}
