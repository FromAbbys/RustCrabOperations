# Local Payload Execution

Two simple techniques for local payload execution: one using a DLL and the other using raw shellcode. These codes are not stealthy and will be easily detected by EDRs or AVs if no encryption or obfuscation methods are applied.

DLL: Loads the malicious DLL into the local process and executes it.

Shellcode: Allocates memory and creates a thread to execute the payload.

# Build

```
cd <directory>
cargo build --release
```
