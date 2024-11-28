use std::fs;
use std::error::Error as Error;
use std::ffi::OsStr;
use std::io::Write;
use std::io;
use filesize::PathExt;
use std::str::FromStr as from_str;
use std::path::PathBuf;

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct File {
    pub size: u64,
    pub path: String,
    pub archived: bool
}

pub fn zip_in_dir(dir: &str) -> Result<bool, Box<dyn Error>> {
    let has_zip = fs::read_dir(dir)?
        .filter_map(|res| res.ok())
        .map(|dir| dir.path())
        .map(|path| path.extension()
            .unwrap_or(OsStr::new(""))
            .to_str()
            .unwrap_or("") == "zip")
        .any(|x| x);

    Ok(has_zip)
}

pub fn get_zip_in_dir(dir: &str) -> Option<PathBuf> {
    let mut dirs = fs::read_dir(dir).ok()?
        .filter_map(|res| res.ok())
        .map(|dir| dir.path());

    dirs.find(|x| x.extension()
        .unwrap_or(OsStr::new(""))
        .to_str()
        .unwrap_or("") == "zip")
}

pub fn print_and_get_input(msg: &str) -> String {
    print!("{}", msg);
    let mut input = String::new();
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).expect("Failed to read line");

    input
}

pub fn get_file_sizes_in_dir(dir: &str) -> Result<Vec<File>, Box<dyn Error>> {
    Ok(
        fs::read_dir(dir)?
        .filter_map(|res| res.ok())
        .map(|dir| dir.path())
        .map(|path| File {
            size: path.size_on_disk().expect("Error reading file size"),
            path: String::from(path.to_str().unwrap()),
            archived: false })
        .collect()
    )
}

pub fn max_file_size_from_input(input: &str) -> Result<u64, Box<dyn Error>> {
    if input.trim().is_empty() {return Ok(10_737_418_240)}

    let len = input.chars().count();
    if len < 3 {return Err("Invalid input".into())};

    let size = u64::from_str(&input[..len-3])?;
    let unit = &input[len-3..];

    match unit.to_lowercase().trim() {
        "kb" => return Ok(size * 1024),
        "mb" => return Ok(size * 1_048_576),
        "gb" => return Ok(size * 1_073_741_824),
        e => return Err(format!("Invalid suffix (KB, MB, GB only), provided {}", e).into())
    }
}
