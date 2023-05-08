// MIT License
//
// Copyright (c) 2021-2023 Brenden Davidson
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

use clap::{ArgAction, Parser};
use std::fs::File;
use std::path::PathBuf;
use std::process::ExitCode;

mod bios;

/// Cross-platform BIOS file renamer for ASUS motherboards
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Path to BIOS file to operate on
    bios_path: PathBuf,

    /// Target output directory for the renamed file
    #[arg(short, long)]
    out_dir: Option<PathBuf>,

    /// Copy the BIOS file instead of moving it
    #[arg(short, long, action = ArgAction::SetTrue)]
    copy: Option<bool>,
}

fn main() -> ExitCode {
    let cli = Cli::parse();

    let bios_path = match cli.bios_path.canonicalize() {
        Ok(path) => path,
        Err(why) => {
            eprintln!("ERROR: {} at path {}", why, cli.bios_path.display());
            return ExitCode::FAILURE;
        }
    };

    let mut bios_file = match File::open(&bios_path) {
        Ok(file) => file,
        Err(why) => {
            eprintln!("ERROR: couldn't open {}: {}", &cli.bios_path.display(), why);
            return ExitCode::FAILURE;
        }
    };

    // Check file validity
    let is_valid = match bios::is_file_valid(&bios_file) {
        Ok(is_valid) => is_valid,
        Err(why) => {
            eprintln!("ERROR: failed to test validity of file: {}", why);
            return ExitCode::FAILURE;
        }
    };

    if !is_valid {
        eprintln!("INVALID FILE: provided file is not the expected size");
        return ExitCode::FAILURE;
    }

    let bios_info = match bios::BiosInfo::from_file(&mut bios_file) {
        Ok(info) => info,
        Err(why) => {
            eprintln!("ERROR: {}", why);
            return ExitCode::FAILURE;
        }
    };
    // Close the file by dropping it
    drop(bios_file);

    // Handle the user setting a target directory
    let mut output_path = match cli.bios_path.parent() {
        Some(dir) => dir.to_owned(),
        None => {
            let mut out = PathBuf::new();
            out.push(".");
            out
        }
    };
    match cli.out_dir {
        Some(dir) => {
            output_path = dir;
        }
        None => {}
    }

    // Rename source file
    output_path.push(bios_info.get_expected_name());
    println!("Output path: {}", &output_path.display());

    let should_copy = cli.copy.unwrap_or(false);

    if should_copy {
        match std::fs::copy(&bios_path, &output_path) {
            Ok(_) => {
                println!("BIOS file copied to: {}", &output_path.display());
            }
            Err(why) => {
                eprintln!("ERROR: Failed to copy file: {}", why);
                return ExitCode::FAILURE;
            }
        };
    } else {
        match std::fs::rename(&bios_path, &output_path) {
            Ok(_) => {
                println!("BIOS file moved to: {}", &output_path.display());
            }
            Err(why) => {
                eprintln!("ERROR: Failed to move file: {}", why);
                return ExitCode::FAILURE;
            }
        };
    }

    ExitCode::SUCCESS
}
