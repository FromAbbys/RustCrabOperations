# Process Injection 

Process injection is a technique that allows code to be executed within the memory space of another process. The code here is written in Rust and demonstrates two different approaches:

- Shellcode → Writes raw shellcode into the target process's memory and forces its execution.

- DLL → Forces the target process to load a malicious DLL.


# Usage

shellcode
```cmd
process_injection.exe <process name>
```

dll
```cmd
process_injection.exe <process name> <dll full path>
```
