use std::fs;
use std::error::Error as Error;
use std::io::Write;
use std::io;
use std::ffi::OsStr;

//      General structure
//
// - Check for presence of .zip [✅]
//      - Warn user if no .zip []
//
// - Ask for desired zip file name root []
// - Ask for maximum zip size (support GB, MB) []
//
// - Unzip []
//
// - Check that largest file does not exceed maximum size []
//
// - Greedy algorithm to build lists of files that do not exceed target []
//      - Set unused files list to all files in zip
//      - Order files from largest to smallest
//      - Set archive-list as empty list of lists
//      - While unused files isn't empty
//          - Create empty archive list
//          - Set current archive size = 0
//          - Iterate over remaining files
//              - If archive size + current file size < max zip:
//                 - rm current file from unused
//                 - add current file path to current archive
//                 - add current file size to current archive size
//          - Add current archive to archive-list

fn main() -> () {

    fn zip_in_dir(dir: &str) -> Result<bool, Box<dyn Error>> {
        Ok(fs::read_dir(dir)?
            .filter_map(|res| res.ok())
            .map(|dir| dir.path())
            .map(|path| path.extension()
                .unwrap_or(OsStr::new(""))
                .to_str()
                .unwrap_or("") == "zip")
            .any(|x| x))
    }

    let has_zip: bool = zip_in_dir("./").expect("Error when reading directory -- maybe run with admin privileges");

    print!("{}", has_zip);
    io::stdout().flush().unwrap();
}
