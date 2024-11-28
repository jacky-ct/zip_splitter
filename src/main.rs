use utils::File;

mod utils;

//      General structure
//
// - Check for presence of .zip [✅]
//      - Warn user if no .zip [✅]
//
// - Ask for desired zip file name root [✅]
// - Ask for maximum zip size (support GB, MB) [✅]
// - Calculate maximum size from input [✅]
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
//
//      Optional stuff
// - Add CLI argument support []
// - Possible to check size of internal files without unzipping?
//      - largest file < max size check without unzipping []

fn main() -> () {

    let has_zip: bool = utils::zip_in_dir("./").expect("Error when reading directory -- maybe run with admin privileges");

    if !has_zip {
        utils::print_and_get_input("No .zip found in this folder, press ENTER to quit");
        std::process::exit(0);
    }

    let archive_name: String = utils::print_and_get_input("Enter name for generated .zip (or ENTER for default \"archive\": ").trim().to_string();
    let mut files: Vec<File> = utils::get_file_sizes_in_dir("./").unwrap();
    files.sort_by_key(|file| file.size);

    let mut max_size: u64 = 0;
    {
        let mut input_msg = "Enter max .zip size (or ENTER for default of 10GB): ";
        loop {
            let max_size_input: String = utils::print_and_get_input(input_msg);

            match utils::max_file_size_from_input(max_size_input.as_str()) {
                Ok(size) => {
                    max_size = size;
                    break;
                    },
                Err(_) => input_msg = "Invalid input, make sure it is a number followed by suffix (e.g. 54MB): "
            }
        }
    }

    print!("{:?}", max_size);
}
