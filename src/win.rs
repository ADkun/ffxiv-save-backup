use std::ffi::OsString;
use std::mem;
use std::ptr;
use std::os::windows::ffi::OsStringExt;
use winapi::um::psapi::{EnumProcesses, GetModuleFileNameExW};
use winapi::um::processthreadsapi::OpenProcess;
use winapi::um::winnt::{PROCESS_QUERY_INFORMATION, PROCESS_VM_READ};
use winapi::shared::minwindef::{DWORD, HMODULE, MAX_PATH};

pub fn get_process_path(process_name: &str) -> Option<String> {
    unsafe {
        let mut processes: [DWORD; 1024] = [0; 1024];
        let mut bytes_needed: DWORD = 0;

        // 获取当前所有进程ID
        if EnumProcesses(processes.as_mut_ptr(), 
                         (mem::size_of_val(&processes) * mem::size_of::<DWORD>()) as DWORD, 
                         &mut bytes_needed) == 0 {
            return None;
        }

        let process_count = bytes_needed / mem::size_of::<DWORD>() as DWORD;

        for &pid in processes.iter().take(process_count as usize) {
            let process_handle = OpenProcess(PROCESS_QUERY_INFORMATION | PROCESS_VM_READ, 0, pid);
            if process_handle.is_null() {
                continue;
            }
            
            let mut module_handle: [HMODULE; 1024] = [ptr::null_mut(); 1024];
            let mut bytes_needed: DWORD = 0;

            // 获取进程的模块句柄
            if winapi::um::psapi::EnumProcessModules(process_handle, module_handle.as_mut_ptr(), 
                                                      (mem::size_of_val(&module_handle) * mem::size_of::<HMODULE>()) as DWORD, 
                                                      &mut bytes_needed) != 0 {
                let module_name = module_handle[0];
                let mut path: [u16; MAX_PATH] = [0; MAX_PATH];
                if GetModuleFileNameExW(process_handle, module_name, path.as_mut_ptr(), MAX_PATH as DWORD) > 0 {
                    let exe_name = OsString::from_wide(&path).to_string_lossy().into_owned();
                    // 检查进程名是否匹配
                    if exe_name.contains(process_name) {
                        // 如果匹配，返回路径
                        return Some(exe_name);
                    }
                }
            }
        }
    }

    None
}
