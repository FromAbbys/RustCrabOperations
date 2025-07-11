# Local Payload Execution

Two simple techniques for local payload execution: one using a DLL and the other using raw shellcode. These codes are not stealthy and will be easily detected by EDRs or AVs if no encryption or obfuscation methods are applied.

DLL: Loads the malicious DLL into the local process and executes it.

Shellcode: Allocates memory and creates a thread to execute the payload.

# Build

```
git clone https://github.com/FromAbbys/RustCrabOperations/tree/94144fae41e9f6a4d29607d56c83f02fd7231a44/Local%20Payload%20Execution
cd <directory>
cargo build --release
```
