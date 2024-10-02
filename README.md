# FF14 存档备份工具

## 介绍

FF14 存档备份工具是一个用于备份FF14存档的工具。
当游戏启动后，工具会自动获取游戏存档的目录，并将存档目录复制到D:\ffxiv_backup目录下，并以日期作为目录名保存。

## 下载方式

目前仅提供源码自行编译。
1. 安装Rust，访问[Rust官网下载](https://www.rust-lang.org/learn/get-started)
2. 安装
    ```
    cargo install https://github.com/ADkun/ffxiv-save-backup
    ```
3. 在Windows系统中，`cargo install`命令会将可执行文件安装到`C:\Users\<用户名>\.cargo\bin`目录下。
   你可以将该目录加入到环境变量PATH中，然后打开cmd，输入`ffxivsavebackup`运行工具。
   也可以打开所在目录，双击运行`ffxivsavebackup.exe`文件。