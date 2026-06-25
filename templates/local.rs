use std::os::unix::fs::PermissionsExt;

const BIN_NAME: &str = "@@BIN_NAME@@";
const BIN_URI: &str = "@@BIN_URI@@";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let path = std::env::current_exe()
        .map_err(|e| format!("failed to get current executable path: {e}"))?;
    let tmp = path.with_extension("tmp");
    std::fs::copy(BIN_URI, &tmp)
        .map_err(|e| format!("failed to copy `{BIN_URI}` to `{}`: {e}", tmp.display()))?;
    std::fs::set_permissions(&tmp, std::fs::Permissions::from_mode(0o755))
        .map_err(|e| format!("failed to set permissions on `{}`: {e}", tmp.display()))?;
    std::fs::rename(&tmp, &path)
        .map_err(|e| format!("failed to rename `{}` to `{}`: {e}", tmp.display(), path.display()))?;
    println!("Successfully installed `{BIN_NAME}`!");
    Ok(())
}
