# BIOS Renamer for ASUS Motherboards

Cross-platform Rust implementation of ASUS' Windows-only BIOS renamer utility.

## Purpose

Modern ASUS motherboards expect BIOS updates to have a specific name; however, ASUS uses more descriptive,
but incompatible, file names in their official downloads. To resolve this, they also provide a small tool which
will rename the file to match what the target motherboard expects.

Unfortunately, the official tool from ASUS is Windows-only. If, for example, the target motherboard is being used for a Linux server,
and one’s other computers are Macs or Linux-based, then the official tool cannot be used to prepare BIOS updates.

That's where this tool comes in! Utilizing Rust and a platform-agnostic design, it can be built and ran on Windows,
macOS, and Linux; providing the exact same functionality as the official tool regardless of OS.

## Build Executable

### Ensure Rust is Installed

https://www.rust-lang.org/tools/install

### CLI

```sh
cargo build
```

or

```sh
cargo build --bin=rename-bios
```

### GUI

```sh
cargo build --bin=rename-bios-gui --features=gui
```

## CLI Usage

### From a File Manager (Windows, some Linux DEs)

Drag the BIOS file to be renamed onto the application icon in your file explorer. The application will automatically rename the file.

### From a terminal (Windows, macOS, Linux)

1. Navigate to the project directory
2. Execute the program by running `cargo run -- <BIOS_FILE>` where `<BIOS_FILE>` is the path to the target file.
3. The file will be renamed and can be found in the project directory. Run `cargo run -- --help` to find out how to control this behavior.
