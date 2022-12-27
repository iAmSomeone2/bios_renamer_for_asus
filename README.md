# BIOS Renamer for Asus Motherboards

Cross-platform Rust implementation of Asus' Windows-only BIOS renamer utility.

## Usage
Before you can use this, you need to compile the code to create an executable.

### Install Rust on Unix-like OSs: Linux, macOS, etc.
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```
### You can even install rust with homebrew on MacOS
```bash
brew install rust 
```

### Confirm Rust is Installed
Once you install rust, you will have the build toolchain. You can confirm that by running these commands.
``` cargo --version
# cargo 1.66.0 (d65d197ad 2022-11-15)

rustc --version
# rustc 1.66.0 (69f9c33d7 2022-12-12)
```

## Build Executable
Go to the root of this repo and run `cargo build`
```bash
cd bios_renamer_for_asus
cargo build
```  

## Using the BIOS renamer 

### From a File Manager (Windows, some Linux DEs)

Drag the BIOS file to be renamed onto the application icon. The application will automatically rename the file.

### From a terminal (Windows, macOS, Linux)

1. Navigate to the directory containing the application.
2. Make the application executable if it is not:
   ``` bash
   chmod +x ./bios_renamer_for_asus 
   ```
3. Call the application with the path to the BIOS file as the first argument:
    ``` bash
    ./bios_renamer_for_asus <path-to-BIOS-file>
    ```
4. Follow the instructions given by the application.

# Easy Script for non-developers:
This will work on linux and mac

```bash
#!/bin/bash
## install rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
## clone repo
git clone https://github.com/iAmSomeone2/bios_renamer_for_asus.git
## create binary
cd bios_renamer_for_asus && cargo build
## Execute Binary, providing firmware path
## eg: ./bios_renamer_for_asus <path-to-BIOS-file>
./target/debug/bios_renamer_for_asus ~/Downloads/ROG-CROSSHAIR-VIII-DARK-HERO-ASUS-4201.CAP
```
Follow the instructions given by the application.