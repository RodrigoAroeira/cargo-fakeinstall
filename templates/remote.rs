use std::os::unix::fs::PermissionsExt;

const BIN_NAME: &str = "@@BIN_NAME@@";
const BIN_URI: &str = "@@BIN_URI@@";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let path = std::env::current_exe()
        .map_err(|e| format!("failed to get current executable path: {e}"))?;
    let tmp = path.with_extension("tmp");
    let cmd = std::process::Command::new("wget")
        .args(["-qO", &tmp, BIN_URI])
        .spawn()
        .map_err(|e| format!("failed to spawn wget for `{BIN_NAME}`: {e}"))?
        .wait()
        .map_err(|e| format!("failed to wait for wget while downloading `{BIN_NAME}`: {e}"))?;
    if !cmd.success() {
        Err(format!("wget failed to download `{BIN_NAME}` binary"))?;
    }
    std::fs::set_permissions(&tmp, std::fs::Permissions::from_mode(0o755))
        .map_err(|e| format!("failed to set permissions on `{}`: {e}", tmp.display()))?;
    std::fs::rename(&tmp, &path)
        .map_err(|e| format!("failed to rename `{}` to `{}`: {e}", tmp.display(), path.display()))?;
    println!("Successfully installed `{BIN_NAME}`!");
    Ok(())
}
