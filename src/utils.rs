use std::fs;
use std::error::Error as Error;
use std::ffi::OsStr;
use std::io::Write;
use std::io;
use filesize::PathExt;

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

pub fn print_and_get_input(msg: &str) -> String {
    print!("{}", msg);
    let mut input = String::new();
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).expect("Failed to read line");

    input
}

pub fn get_file_sizes_in_dir(dir: &str) -> Result<Vec<(u64, String)>, Box<dyn Error>> {
    Ok(
        fs::read_dir(dir)?
        .filter_map(|res| res.ok())
        .map(|dir| dir.path())
        .map(|path| (path.size_on_disk().expect("Error reading file size"), String::from(path.to_str().unwrap())))
        .collect()
    )
}
