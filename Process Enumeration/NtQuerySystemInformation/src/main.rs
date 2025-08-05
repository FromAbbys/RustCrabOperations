use std::{ffi::c_void,ptr::null_mut};
use windows_sys::{
    Wdk::{
        System::{
            SystemInformation::{
                NtQuerySystemInformation, SystemProcessInformation
            }
    }
    },
    Win32::{
        Foundation::{
            UNICODE_STRING, HANDLE, GetLastError, NTSTATUS
        },
        }
};


#[allow(non_snake_case, non_camel_case_types)]
#[repr(C)]
struct SYSTEM_PROCESS_INFORMATION {
    pub NextEntryOffset: u32,
    pub NumberOfThreads: u32,
    pub Reserved1: [u8; 48],
    pub ImageName: UNICODE_STRING,
    pub BasePriority: i32,
    pub UniqueProcessId: HANDLE,
    pub Reserved2: *mut c_void,
    pub HandleCount: u32,
    pub SessionId: u32,
    pub Reserved3: *mut c_void,
    pub PeakVirtualSize: usize,
    pub VirtualSize: usize,
    pub Reserved4: u32,
    pub PeakWorkingSetSize: usize,
    pub WorkingSetSize: usize,
    pub Reserved5: *mut c_void,
    pub QuotaPagedPoolUsage: usize,
    pub Reserved6: *mut c_void,
    pub QuotaNonPagedPoolUsage: usize,
    pub PagefileUsage: usize,
    pub PeakPagefileUsage: usize,
    pub PrivatePageCount: usize,
    pub Reserved7: [i64; 6],
}

#[allow(non_snake_case)]
fn NT_SUCCESS(nt_status: NTSTATUS) -> bool {
    nt_status >= 0
}

fn main() {
    unsafe {
        let mut u_return_1 = 0u32;
        let mut u_return_2 = 0u32;

    
        let _ntquery = NtQuerySystemInformation(
            SystemProcessInformation,
            null_mut(),
            0,
            &mut u_return_1
        );

        let mut buffer = vec![0u8;u_return_1 as usize];
        let address = buffer.as_mut_ptr() as *mut SYSTEM_PROCESS_INFORMATION;

        let status = NtQuerySystemInformation(
            SystemProcessInformation,
            address.cast(),
            u_return_1,
            &mut u_return_2
        );

        if !NT_SUCCESS(status) {
            println!("[ ! ] NtQuerySystemInformation Failed with error : {}", GetLastError());
            return;
        }

        let mut current = address;

        loop {
            let proc_info = &*current;
            let process_pid = proc_info.UniqueProcessId as u32;

            let process_name = if proc_info.ImageName.Length > 0 {
                let name_utf16 = std::slice::from_raw_parts(
                    proc_info.ImageName.Buffer,
                    (proc_info.ImageName.Length / 2) as usize
                );
                String::from_utf16_lossy(name_utf16)
            } else {
                "System Idle Process".to_string()
            };

            println!("[ PID {:>5} ] => {:<40}", process_pid, process_name);

            if proc_info.NextEntryOffset == 0 {
                break
            }

            current = (proc_info as *const SYSTEM_PROCESS_INFORMATION as *const u8)
            .add(proc_info.NextEntryOffset as usize) as *mut SYSTEM_PROCESS_INFORMATION;
        }
    }
}