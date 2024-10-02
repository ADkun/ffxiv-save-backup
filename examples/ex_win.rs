use ffsavebackup::win;

fn main() {
    let exe = "svchost.exe";

    let path = win::get_process_path(exe).unwrap();

    println!("{path}");
}

