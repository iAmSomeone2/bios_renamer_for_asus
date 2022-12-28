// MIT License
//
// Copyright (c) 2021-2022 Brenden Davidson
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

use clap::Parser;
use std::fs::File;
use std::path::PathBuf;
use std::process::exit;

mod bios;

/// Cross-platform BIOS file renamer for ASUS motherboards
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Path to BIOS file to operate on
    bios_path: PathBuf,
    // Target output directory for the renamed file
    // #[arg(short, long)]
    // out_dir: Option<PathBuf>,

    // Copy the BIOS file instead of moving it
    // #[arg(short, long, action = ArgAction::SetTrue)]
    // copy: Option<bool>,
}

fn main() {
    let cli = Cli::parse();

    let bios_path = match cli.bios_path.canonicalize() {
        Ok(path) => path,
        Err(why) => {
            eprintln!("ERROR: {} at path {}", why, cli.bios_path.display());
            exit(1);
        }
    };

    let mut bios_file = match File::open(&bios_path) {
        Ok(file) => file,
        Err(why) => {
            eprintln!("ERROR: couldn't open {}: {}", &cli.bios_path.display(), why);
            exit(1);
        }
    };

    let bios_info = match bios::BiosInfo::from_file(&mut bios_file) {
        Ok(info) => info,
        Err(why) => {
            eprintln!("ERROR: {}", why);
            exit(2);
        }
    };
    // Close the file by dropping it
    drop(bios_file);

    // TODO: Add support for target directory and copy mode

    // Rename source file
    let output_path = PathBuf::from(bios_info.get_expected_name());
    match std::fs::rename(&bios_path, &output_path) {
        Ok(_) => {
            println!("File renamed to: {}", &output_path.display());
        }
        Err(why) => {
            eprintln!("ERROR: Failed to rename file: {}", why);
            exit(3);
        }
    };
}
