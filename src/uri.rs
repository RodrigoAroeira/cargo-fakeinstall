use std::str::FromStr;

use expanduser::expanduser;

#[derive(Clone)]
pub enum Uri {
    Remote(String),
    Local(String),
}

impl FromStr for Uri {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some(path) = s.strip_prefix("file://") {
            let expanded = expanduser(path).map_err(|e| e.to_string())?;
            let absolute = std::path::absolute(expanded).map_err(|e| e.to_string())?;
            Ok(Uri::Local(absolute.display().to_string()))
        } else {
            Ok(Uri::Remote(s.to_string()))
        }
    }
}
