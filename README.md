# grep.rs
This repository features my own implementation of grep (global regular expression print) written in Rust.

# Syntax
```
# compile with rustc and execute executable
rustc <PATH TO SCRIPT>.rs
<PATH TO SCRIPT>.exe <PATH> <PARAMETER> <STRING>
# example: .\main.exe . -s "json"

# compile with cargo (navigate to main folder after cargo init and lacing script in src folder)
cargo build
cargo run <PARAMETERS>
# example: cargo run . -s "json"
```