//      General structure
//
// - Check for presence of .zip []
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
fn main() {
    println!("Hello, world!");
}
