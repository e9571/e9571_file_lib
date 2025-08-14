use std::fs::{self, File, OpenOptions};
use std::io::{self, Read, Write};
use std::path::{Path, PathBuf};
use chksum_md5 as md5;
use lazy_static::lazy_static;
use std::sync::Mutex;
use std::collections::HashMap;



lazy_static! {
    pub static ref FILE_LIST: Mutex<Vec<String>> = Mutex::new(Vec::new());
    pub static ref FILE_INFO_LIST: Mutex<HashMap<String, String>> = Mutex::new(HashMap::new());
}


/// 检查文件或目录是否存在 (assumed Exists)
fn exists(path: &str) -> bool {
    fs::metadata(path).is_ok()
}


pub mod e9571_file_lib {
    use super::*;

    /// 获取文件夹文件列表 (ListFile)
    /// Returns a map of file paths to their parent directories
    pub fn list_file(myfolder: &str) -> Result<HashMap<String, String>, io::Error> {
        let mut out_put = HashMap::new();
        let entries = fs::read_dir(myfolder)?;

        for entry in entries {
            let file = entry?;
            let path = file.path();
            let path_str = path.to_str().unwrap_or("").to_string();
            let parent_str = myfolder.to_string();
            out_put.insert(path_str, parent_str);
        }
        Ok(out_put)
    }

    /// 遍历文件路径并添加到全局列表 (Visit)
    /// Adds file paths to the global File_info_list
    pub fn visit(path: &str) {
        let mut file_info_list = FILE_INFO_LIST.lock().unwrap();
        file_info_list.insert(path.to_string(), path.to_string());
    }

    /// 获取文件 MD5 哈希 (File_MD5)
    /// Returns the uppercase MD5 hash of a file
    pub fn file_md5(path: &str) -> String {
        let file = match File::open(path) {
            Ok(file) => file,
            Err(e) => {
                println!("Open: {}", e);
                return String::new();
            }
        };

        match md5::chksum(file) {
            Ok(digest) => digest.to_hex_uppercase(),
            Err(e) => {
                println!("Checksum: {}", e);
                String::new()
            }
        }
    }

    /// 重命名文件 (File_Rename)
    /// Renames a file and prints success or error
    pub fn file_rename(source_file: &str, target_file: &str) {
        match fs::rename(source_file, target_file) {
            Ok(_) => println!("file rename OK!"),
            Err(e) => println!("file rename Error: {}", e),
        }
    }

    /// 遍历文件夹获取文件列表 (GetFilelist)
    /// Returns a list of all file paths (excluding directories)
    pub fn get_filelist(path: &str) -> Vec<String> {
        let mut path_tmp = Vec::new();
        if let Err(e) = walk_dir_files(path, &mut path_tmp) {
            println!("filepath.Walk() returned: {}", e);
        }
        path_tmp
    }

    fn walk_dir_files(path: &str, paths: &mut Vec<String>) -> io::Result<()> {
        for entry in fs::read_dir(path)? {
            let file = entry?;
            let file_path = file.path();
            if file.file_type()?.is_dir() {
                walk_dir_files(file_path.to_str().unwrap_or(""), paths)?;
            } else {
                paths.push(file_path.to_str().unwrap_or("").to_string());
            }
        }
        Ok(())
    }

    /// 获取指定目录及子目录下的文件，匹配后缀 (WalkDir)
    /// Returns a list of file paths with the specified suffix (case-insensitive)
    pub fn walk_dir(dir_pth: &str, suffix: &str) -> Result<Vec<String>, io::Error> {
        let mut files = Vec::new();
        let suffix_upper = suffix.to_uppercase();
        walk_dir_recursive(dir_pth, &suffix_upper, &mut files)?;
        Ok(files)
    }

    fn walk_dir_recursive(dir_pth: &str, suffix: &str, files: &mut Vec<String>) -> io::Result<()> {
        for entry in fs::read_dir(dir_pth)? {
            let file = entry?;
            let path = file.path();
            if file.file_type()?.is_dir() {
                walk_dir_recursive(path.to_str().unwrap_or(""), suffix, files)?;
            } else if path.file_name()
                .and_then(|name| name.to_str())
                .map(|name| name.to_uppercase().ends_with(suffix))
                .unwrap_or(false) {
                files.push(path.to_str().unwrap_or("").to_string());
            }
        }
        Ok(())
    }

    /// 递归获取所有文件，添加到全局 File_list (GetFiles)
    /// Populates the global FILE_LIST with file paths
    pub fn get_files(folder: &str) {
        if let Err(e) = get_files_recursive(folder) {
            println!("read dir fail: {}", e);
        }
    }

    fn get_files_recursive(folder: &str) -> io::Result<()> {
        let entries = fs::read_dir(folder)?;
        let mut file_list = FILE_LIST.lock().unwrap();
        for entry in entries {
            let file = entry?;
            let path = file.path();
            let path_str = path.to_str().unwrap_or("").to_string();
            if file.file_type()?.is_dir() {
                get_files_recursive(&path_str)?;
            } else {
                file_list.push(path_str);
            }
        }
        Ok(())
    }

    /// 获取指定文件夹下所有文件，跳过目录 (GetAllFile)
    /// Returns a list of file paths, excluding directories
    pub fn get_all_file(pathname: &str, mut s: Vec<String>) -> Result<Vec<String>, io::Error> {
        let entries = fs::read_dir(pathname)?;
        for entry in entries {
            let file = entry?;
            if !file.file_type()?.is_dir() {
                let full_name = format!("{}/{}", pathname, file.file_name().to_str().unwrap_or(""));
                s.push(full_name);
            }
        }
        Ok(s)
    }

    /// 获取指定文件夹下所有文件夹 (GetDirList)
    /// Returns a list of directory paths
    pub fn get_dir_list(dirpath: &str) -> Result<Vec<String>, io::Error> {
        let mut dir_list = Vec::new();
        walk_dir_dirs(dirpath, &mut dir_list)?;
        Ok(dir_list)
    }

    fn walk_dir_dirs(path: &str, dirs: &mut Vec<String>) -> io::Result<()> {
        for entry in fs::read_dir(path)? {
            let file = entry?;
            if file.file_type()?.is_dir() {
                let path_str = file.path().to_str().unwrap_or("").to_string();
                dirs.push(path_str.clone());
                walk_dir_dirs(&path_str, dirs)?;
            }
        }
        Ok(())
    }

    /// 拷贝单个文件 (CopyFile)
    /// Copies a file from src to dst, returns bytes written and error
    pub fn copy_file(src_name: &str, dst_name: &str) -> io::Result<(i64, io::Error)> {
        let mut src = match File::open(src_name) {
            Ok(file) => file,
            Err(e) => return Ok((0, e)),
        };
        let mut dst = match OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(dst_name)
        {
            Ok(file) => file,
            Err(e) => return Ok((0, e)),
        };
        match io::copy(&mut src, &mut dst) {
            Ok(written) => Ok((written as i64, io::Error::new(io::ErrorKind::Other, "No error"))),
            Err(e) => Ok((0, e)),
        }
    }

    /// 拷贝单个文件 (CopyFile1)
    /// Copies a file from src to dst, ensures sync
    pub fn copy_file1(src: &str, dst: &str) -> io::Result<()> {
        let mut src_file = File::open(src)?;
        let mut dst_file = File::create(dst)?;
        io::copy(&mut src_file, &mut dst_file)?;
        dst_file.sync_all()?;
        Ok(())
    }

    /// 递归拷贝目录 (CopyDir)
    /// Copies a directory and its contents recursively
    pub fn copy_dir(src: &str, dst: &str) -> io::Result<()> {
        let entries = fs::read_dir(src)?;
        for entry in entries {
            let file = entry?;
            let src_path = file.path();
            let dst_path = Path::new(dst).join(file.file_name());
            if file.file_type()?.is_dir() {
                fs::create_dir_all(&dst_path)?;
                copy_dir(src_path.to_str().unwrap_or(""), dst_path.to_str().unwrap_or(""))?;
            } else {
                copy_file1(src_path.to_str().unwrap_or(""), dst_path.to_str().unwrap_or(""))?;
            }
        }
        Ok(())
    }

    /// 判断文件是否存在 (CheckFileIsExist)
    /// Returns true if the file exists
    pub fn check_file_is_exist(filename: &str) -> bool {
        fs::metadata(filename).is_ok()
    }

    /// 判断路径是否为文件夹 (IsDir)
    /// Returns true if the path is a directory
    pub fn is_dir(name: &str) -> bool {
        fs::metadata(name)
            .map(|metadata| metadata.is_dir())
            .unwrap_or(false)
    }

    /// 判断路径是否为文件 (IsFile)
    /// Returns true if the path is a file
    pub fn is_file(path: &str) -> bool {
        !is_dir(path)
    }

    /// 新建文件，自动创建目录 (Create_New_File)
    /// Creates a directory if it doesn’t exist, returns the path
    pub fn create_new_file(file_name: &str) -> Result<String, io::Error> {
        if check_file_is_exist(file_name) {
            return Ok(file_name.to_string());
        }
        fs::create_dir(file_name)?;
        Ok(file_name.to_string())
    }

    /// 递归删除指定目录下的所有文件 (DeleteFilesInDir)
    /// Deletes all files in a directory, excluding directories
    pub fn delete_files_in_dir(dir: &str) -> io::Result<()> {
        for entry in fs::read_dir(dir)? {
            let file = entry?;
            if !file.file_type()?.is_dir() {
                let path = file.path();
                fs::remove_file(&path).map_err(|e| {
                    io::Error::new(
                        io::ErrorKind::Other,
                        format!("删除文件 {} 时出错: {}", path.display(), e),
                    )
                })?;
                println!("已删除文件: {}", path.display());
            }
        }
        Ok(())
    }
}