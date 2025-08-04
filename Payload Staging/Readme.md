# Payload Staging

Demonstrates two different techniques for storing the payload outside the main binary.
In this repo, we have stored it in a web server or a registry key, but it can be anything.

# Usage
Registry
```
cd Registry\
cargo build --release
target\release\<exe file>
```

Web
Remember to modify the IP address where your payload is stored
```
cd Web\
cargo build --release
target\release\<exe file>
```
