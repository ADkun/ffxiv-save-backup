use std::io::{self, Write};
use std::path::Path;
use std::error::Error;

/// 按任意键继续
pub fn wait_for_continue() {
    println!("按任意键继续...");

    // 使用 flush 确保提示信息立即打印
    io::stdout().flush().unwrap();

    // 等待用户输入
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
}

/// 获取给定路径的父路径
pub fn get_parent_path<T: AsRef<Path>>(path: T) -> String {
    path.as_ref().parent().unwrap().to_str().unwrap().to_string()
}

/// 获取当前时间，并格式化为字符串 yyyy-mm-dd hh:mm:ss
pub fn get_current_time_str() -> String {
    let now = chrono::Local::now();
    now.format("%Y%m%d_%H%M%S").to_string()
}

/// 给定路径，如果路径不存在，则创建目录
pub fn create_dir<T: AsRef<Path>>(path: T) -> Result<(), Box<dyn Error>> {
    let path = path.as_ref();
    if !path.exists() {
        std::fs::create_dir_all(path)?;
    }
    Ok(())
}

/// 给两个目录，将第一个目录复制到第二个目录里
pub fn copy_dir<T: AsRef<Path>, U: AsRef<Path>>(from: T, to: U) -> Result<(), String> {
    let from_path = from.as_ref();
    let to_path = to.as_ref();

    if !from_path.is_dir() {
        return Err(format!("源路径不是目录: {}", from_path.display()));
    }

    if !to_path.is_dir() {
        return Err(format!("目标路径不是目录: {}", to_path.display()));
    }

    let mut options = fs_extra::dir::CopyOptions::new();
    options.overwrite = true;
    options.copy_inside = true;
    fs_extra::dir::copy(from_path, to_path, &options).map_err(|e| e.to_string())?;

    Ok(())
}
