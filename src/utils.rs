use std::fs;
use std::error::Error as Error;
use std::ffi::OsStr;

pub fn zip_in_dir(dir: &str) -> Result<bool, Box<dyn Error>> {
    Ok(fs::read_dir(dir)?
        .filter_map(|res| res.ok())
        .map(|dir| dir.path())
        .map(|path| path.extension()
            .unwrap_or(OsStr::new(""))
            .to_str()
            .unwrap_or("") == "zip")
        .any(|x| x))
}

