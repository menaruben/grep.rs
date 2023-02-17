# grep.rs
This repository features my own implementation of grep (global regular expression print) written in Rust. This is not finished at all since grep has a lot of parameters I need to implement into my script! This might take a while but will become more and more useful for operating systems without grep! :)

For now this is equivalent with ```ls | grep <PARAMETER> ``` since the ls is hardcoded inside my script (because I had a very specific usecase) but I will change this in the near future in order to make it more like the actual grep. 

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

# Parameters
- -s, --contains <string>  = only show elements containing "string"
- -v, --invert-match <string> = only show elements not containing "string"
- -c, --count <string> = count how many times a string occurs