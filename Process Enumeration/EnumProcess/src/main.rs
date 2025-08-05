use windows_sys::Win32::{
        Foundation::{CloseHandle, GetLastError, FALSE, HMODULE},
        System::{
            ProcessStatus::{EnumProcessModules, EnumProcesses, GetModuleBaseNameW, }, 
            Threading::{OpenProcess,PROCESS_QUERY_INFORMATION, PROCESS_VM_READ}
        }
    };
use std::os::{raw::c_void, windows::ffi::OsStringExt};
use std::ptr::null_mut;
fn main() {
    unsafe {
        let mut adw_process : Vec<u32> = vec![0; 2048];
        let mut dw_return_len1 = 0;

        let first_exec = EnumProcesses (
            adw_process.as_mut_ptr(),
            (adw_process.len() * std::mem::size_of::<u32>()) as u32, 
            &mut dw_return_len1
        );
        
        if first_exec == 0 {
            println!("[ ! ] EnumProcesses Exit With Error : {}", GetLastError());
            std::process::exit(0);
        }

        let dw_number_of_pids = dw_return_len1 / std::mem::size_of::<u32>() as u32;

        println!("[ + ] Total Number of Processes Detected : {}", dw_number_of_pids);
        println!();

        for pids in 0..dw_number_of_pids {

            let h_process = OpenProcess(
                PROCESS_QUERY_INFORMATION | PROCESS_VM_READ,
                FALSE,
                adw_process[pids as usize]
            );

            if h_process.is_null() {
                continue;
            }
            
            let mut sz_proc : Vec<u16> = vec![0;260];
            let size = sz_proc.len() as u32;
            let mut h_module: *mut c_void = null_mut();
            let mut dw_return_len2 : u32 = 0;

            EnumProcessModules(
                h_process,
                &mut h_module,
                std::mem::size_of::<HMODULE>() as u32,
                &mut dw_return_len2
            );
            
            GetModuleBaseNameW(
                h_process,
                h_module,
                sz_proc.as_mut_ptr(),
                size
            );

            let len = sz_proc.iter().position(|&c| c == 0).unwrap_or(sz_proc.len());
            let name = std::ffi::OsString::from_wide(&sz_proc[..len]).to_string_lossy().to_string();
            println!("[ PID {:>5} ] => {:<40}", adw_process[pids as usize], name);
            CloseHandle(h_process);
            CloseHandle(h_module);
        }
    }
}