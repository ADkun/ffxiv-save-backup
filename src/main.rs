use ffsavebackup::{win, util};
use std::path::PathBuf;

const GAME_NAME: &str = "ffxiv_dx11.exe";
const BACKUP_DIR: &str = "D:\\ffxiv_backup";

fn app() {
    let game_path = match win::get_process_path(GAME_NAME) {
        Some(path) => path,
        None => {
            eprintln!("没有找到进程");
            util::wait_for_continue();
            return;
        }
    };
    println!("游戏路径: {game_path}");

    let parent_path = util::get_parent_path(&game_path);
    println!("游戏目录: {parent_path}");

    let mut save_path = PathBuf::from(&parent_path);
    save_path.push("My Games");
    save_path.push("FINAL FANTASY XIV - A Realm Reborn");
    println!("存档目录: {}", save_path.display());

    let mut backup_path = PathBuf::from(BACKUP_DIR);
    backup_path.push(util::get_current_time_str());
    println!("备份目录: {}", backup_path.display());

    match util::create_dir(&backup_path) {
        Ok(_) => {}
        Err(e) => {
            eprintln!("创建备份目录失败，错误原因：{e}");
            return;
        }
    }

    match util::copy_dir(&save_path, &backup_path) {
        Ok(_) => println!("备份成功"),
        Err(e) => eprintln!("备份失败: {}", e),
    }
}

fn main() {
    app();
}
