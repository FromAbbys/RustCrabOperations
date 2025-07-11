use windows_sys::{
    Win32::{Foundation::*, System::{LibraryLoader::{LoadLibraryA}, Threading::*}}
};
use std::env::args;
use std::io::*;

fn main() {
    // Collect the DLL path from commandline arguments
    let args : Vec<String> = args().collect();

    if args.len() < 2 {
        println!("[ + ] Missing Argument; DLL payload to run");
        std::process::exit(1);
    }
    unsafe {
    println!("[ + ] Injecting {} to the local process of pid: {}", args[1], GetCurrentProcessId());
    
    // Load DLL into the process
    println!("[ + ] Loading DLL...");
    if LoadLibraryA(args[1].as_ptr()) == 0 
    {
        println!("[ ! ] LoadLibraryA Failed with error : {}", GetLastError());
        std::process::exit(1);
    }

    println!("[ + ] Done");
    let mut input_buf : [u8;1] = [0];
    println!("[ + ] Press Enter to quit");
    stdin().read_exact(&mut input_buf).unwrap();  
    }
}

