# e9571_file_lib Usage Examples

This document demonstrates the usage of the `e9571_file_lib` module in a Rust program, designed for casino-related file operations such as copying, renaming, and managing directories or logs for betting records.

## Source Code Example

Below is a Rust program showcasing various file management functions from the `e9571_file_lib` module. The code creates, reads, and manipulates files and directories, handling errors and leveraging global variables (`FILE_INFO_LIST`, `FILE_LIST`) for file metadata.

```rust
use e9571_file_lib::e9571_file_lib::*;
use e9571_file_lib::{FILE_INFO_LIST, FILE_LIST};
use std::fs;

fn main() {
    // Prepare test file
    fs::write(r"D:\test.txt", "hello").expect("Failed to create test file");

    // Example 1: CopyFile (Commented)
    println!("=== CopyFile ===");
    /*
    match copy_file(r"D:\test.txt", r"D:\test_copy.txt") {
        Ok((written, _)) => println!("Bytes written: {}", written),
        Err((_, e)) => println!("Error: {}", e),
    }
    */

    // Example 2: File_MD5
    println!("\n=== File_MD5 ===");
    let md5 = file_md5(r"D:\test.txt");
    println!("MD5 of test.txt: {}", md5);
    assert_eq!(md5, "5D41402ABC4B2A76B9719D911017C592");

    // Example 3: Convert_appoint_number (Commented)
    /*
    println!("\n=== Convert_appoint_number ===");
    let timestamp = "1626187200"; // 2021-07-13 12:00:00 UTC
    println!("Timestamp {}: {}", timestamp, convert_appoint_number(timestamp));
    */

    // Example 4: ListFile
    println!("\n=== ListFile ===");
    match list_file(r"D:\test_dir") {
        Ok(map) => println!("File list: {:?}", map),
        Err(e) => println!("Error: {}", e),
    }

    // Example 5: Visit
    println!("\n=== Visit ===");
    visit(r"D:\test.txt");
    let file_info_list = FILE_INFO_LIST.lock().unwrap();
    println!("File_info_list: {:?}", *file_info_list);

    // Example 6: File_Rename
    println!("\n=== File_Rename ===");
    file_rename(r"D:\test.txt", r"D:\test_renamed.txt");

    // Example 7: GetFilelist
    println!("\n=== GetFilelist ===");
    println!("Files: {:?}", get_filelist(r"D:\test_dir"));

    // Example 8: WalkDir
    println!("\n=== WalkDir ===");
    match walk_dir(r"D:\test_dir", ".txt") {
        Ok(files) => println!("Text files: {:?}", files),
        Err(e) => println!("Error: {}", e),
    }

    // Example 9: GetFiles
    println!("\n=== GetFiles ===");
    get_files(r"D:\test_dir");
    let file_list = FILE_LIST.lock().unwrap();
    println!("File_list: {:?}", *file_list);

    // Example 10: GetAllFile
    println!("\n=== GetAllFile ===");
    match get_all_file(r"D:\test_dir", Vec::new()) {
        Ok(files) => println!("Files: {:?}", files),
        Err(e) => println!("Error: {}", e),
    }

    // Example 11: GetDirList
    println!("\n=== GetDirList ===");
    match get_dir_list(r"D:\test_dir") {
        Ok(dirs) => println!("Directories: {:?}", dirs),
        Err(e) => println!("Error: {}", e),
    }

    // Example 12: CopyFile1
    println!("\n=== CopyFile1 ===");
    if let Err(e) = copy_file1(r"D:\test_renamed.txt", r"D:\test_copy1.txt") {
        println!("Error: {}", e);
    }

    // Example 13: CopyDir
    println!("\n=== CopyDir ===");
    if let Err(e) = copy_dir(r"D:\test_dir", r"D:\test_dir_copy") {
        println!("Error: {}", e);
    }

    // Example 14: CheckFileIsExist
    println!("\n=== CheckFileIsExist ===");
    println!("Exists test_renamed.txt: {}", check_file_is_exist(r"D:\test_renamed.txt"));

    // Example 15: IsDir
    println!("\n=== IsDir ===");
    println!("Is test_dir a directory: {}", is_dir(r"D:\test_dir"));

    // Example 16: IsFile
    println!("\n=== IsFile ===");
    println!("Is test_renamed.txt a file: {}", is_file(r"D:\test_renamed.txt"));

    // Example 17: Create_New_File
    println!("\n=== Create_New_File ===");
    match create_new_file(r"D:\new_dir") {
        Ok(path) => println!("Created: {}", path),
        Err(e) => println!("Error: {}", e),
    }

    // Example 18: DeleteFilesInDir
    println!("\n=== DeleteFilesInDir ===");
    if let Err(e) = delete_files_in_dir(r"D:\test_dir") {
        println!("Error: {}", e);
    }
}
```

## Explanation of Functions

The `e9571_file_lib` module provides utility functions for file and directory operations, ideal for casino applications to manage logs, betting records, or backups.

1. **`copy_file`** (Commented):
   - Copies a file from source to destination, returning the number of bytes written.
   - **Use Case**: Backing up betting logs.

2. **`file_md5`**:
   - Computes the MD5 hash of a file's contents.
   - **Use Case**: Verifying the integrity of game logs or transaction records.

3. **`convert_appoint_number`** (Commented):
   - Converts a timestamp to a specific format (assumed).
   - **Use Case**: Formatting timestamps for bet records.

4. **`list_file`**:
   - Lists files in a directory, returning a map of file metadata.
   - **Use Case**: Retrieving a list of log files.

5. **`visit`**:
   - Visits a file and stores its metadata in `FILE_INFO_LIST`.
   - **Use Case**: Collecting file information for audit purposes.

6. **`file_rename`**:
   - Renames a file from source to destination path.
   - **Use Case**: Archiving old betting logs.

7. **`get_filelist`**:
   - Retrieves a list of files in a directory.
   - **Use Case**: Listing available game logs.

8. **`walk_dir`**:
   - Recursively walks a directory and returns files with a specified extension.
   - **Use Case**: Finding all `.txt` log files in a directory.

9. **`get_files`**:
   - Populates `FILE_LIST` with files from a directory.
   - **Use Case**: Tracking files for batch processing.

10. **`get_all_file`**:
    - Recursively retrieves all files in a directory.
    - **Use Case**: Generating a complete file inventory.

11. **`get_dir_list`**:
    - Lists directories in a specified path.
    - **Use Case**: Organizing log directories by date.

12. **`copy_file1`**:
    - Copies a file (alternative implementation to `copy_file`).
    - **Use Case**: Creating backup copies of transaction files.

13. **`copy_dir`**:
    - Copies an entire directory to a new location.
    - **Use Case**: Backing up a directory of betting records.

14. **`check_file_is_exist`**:
    - Checks if a file exists.
    - **Use Case**: Verifying the presence of a log file.

15. **`is_dir`**:
    - Checks if a path is a directory.
    - **Use Case**: Validating directory paths for storage.

16. **`is_file`**:
    - Checks if a path is a file.
    - **Use Case**: Confirming a betting log is a file.

17. **`create_new_file`**:
    - Creates a new file or directory.
    - **Use Case**: Initializing new log files or folders.

18. **`delete_files_in_dir`**:
    - Deletes all files in a directory.
    - **Use Case**: Clearing temporary or outdated logs.

## Casino Scenario Usage

These functions are ideal for casino applications, such as:
- **Log Management**: Creating, copying, or deleting betting logs (`create_new_file`, `copy_file1`, `delete_files_in_dir`).
- **Data Integrity**: Verifying file contents with MD5 hashes (`file_md5`).
- **File Organization**: Listing and managing log files or directories (`list_file`, `get_filelist`, `get_dir_list`).
- **Backup Operations**: Copying files or directories for archival (`copy_file1`, `copy_dir`).
- **File Validation**: Checking file or directory existence (`check_file_is_exist`, `is_file`, `is_dir`).

## Example Output

The output depends on the file system and `e9571_file_lib` implementation. Assuming the operations succeed (as of 07:06 PM JST on August 14, 2025), an example output might look like:

```
=== CopyFile ===
(Commented out)

=== File_MD5 ===
MD5 of test.txt: 5D41402ABC4B2A76B9719D911017C592

=== Convert_appoint_number ===
(Commented out)

=== ListFile ===
File list: {"test.txt": {...}}

=== Visit ===
File_info_list: [{"path": "D:\\test.txt", "size": 5, ...}]

=== File_Rename ===
(Performed rename from test.txt to test_renamed.txt)

=== GetFilelist ===
Files: ["test_renamed.txt"]

=== WalkDir ===
Text files: ["D:\\test_dir\\test_renamed.txt"]

=== GetFiles ===
File_list: ["D:\\test_dir\\test_renamed.txt"]

=== GetAllFile ===
Files: ["D:\\test_dir\\test_renamed.txt"]

=== GetDirList ===
Directories: ["D:\\test_dir\\sub_dir"]

=== CopyFile1 ===
(Copy completed)

=== CopyDir ===
(Directory copy completed)

=== CheckFileIsExist ===
Exists test_renamed.txt: true

=== IsDir ===
Is test_dir a directory: true

=== IsFile ===
Is test_renamed.txt a file: true

=== Create_New_File ===
Created: D:\new_dir

=== DeleteFilesInDir ===
(Files deleted)
```

## Notes
- **File System**: The code assumes write access to `D:\test.txt` and related paths. Ensure paths are valid and accessible.
- **Error Handling**: The code uses `Result` and `expect` for robust error handling. In production, consider logging errors instead of panicking.
- **Casino Context**: Functions are suitable for managing logs, backups, or transaction records in a casino system.
- **GitHub Rendering**: This Markdown uses Rust syntax highlighting, clear headings, and structured explanations for optimal display.
- **Global Variables**: `FILE_INFO_LIST` and `FILE_LIST` use mutexes for thread safety; ensure proper synchronization in concurrent scenarios.
- **Commented Code**: The `copy_file` and `convert_appoint_number` examples are commented out but retained for reference.