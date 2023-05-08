# BIOS Renamer for ASUS Motherboards

Cross-platform Rust implementation of ASUS' Windows-only BIOS renamer utility.

## Usage
Before you can use this, you need to compile the code to create an executable.

### Install Rust on Unix-like OSs: Linux, macOS, etc.

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### Confirm Rust is Installed

Once you install Rust, you will have the build toolchain. You can confirm that by running these commands.

```sh
cargo --version
# cargo 1.66.0 (d65d197ad 2022-11-15)

rustc --version
# rustc 1.66.0 (69f9c33d7 2022-12-12)
```

## Build Executable

Go to the root of this repo and run `cargo build`

```sh
cd bios_renamer_for_asus
cargo build
```  

## Using the BIOS renamer

### From a File Manager (Windows, some Linux DEs)

Drag the BIOS file to be renamed onto the application icon. The application will automatically rename the file.

### From a terminal (Windows, macOS, Linux)

1. Navigate to the project directory
2. Execute the program by running `cargo run -- <BIOS_FILE>` where `<BIOS_FILE>` is the path to the target file.
3. The file will be renamed and can be found in the project directory. Run `cargo run -- --help` to find out how to control this behavior.
