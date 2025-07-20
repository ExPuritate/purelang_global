use std::path::PathBuf;
use std::str::FromStr;

pub fn get_stdlib_dir() -> crate::Result<String> {
    let home = std::env::var("PURELANG_HOME")?;
    let mut buf = PathBuf::from_str(&home)?;
    buf.push("Library");
    Ok(buf.to_str().unwrap().to_owned())
}
