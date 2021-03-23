# BIOS Renamer for Asus Motherboards

Cross-platform Rust implementation of Asus' Windows-only BIOS renamer utility.

## Usage

### From a File Manager (Windows, some Linux DEs)

Drag the BIOS file to be renamed onto the application icon. The application will automatically rename
the file.

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
