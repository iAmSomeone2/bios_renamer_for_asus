// MIT License
//
// Copyright (c) 2021 Brenden Davidson
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

use std::env;
use std::fs;
use std::path::Path;
use std::ffi::OsStr;
use std::process::exit;

fn rename_file(file_name: &String) -> Option<String> {
    //! Takes the original file name of the form "{}-{}-\<chipset\>-{}-{}-ASUS-\<version\>" -- where '{}'
    //! represents a word in the product name -- and converts it to a form that the BIOS will
    //! recognize.

    let segments: Vec<&str> = file_name.split("-").collect();
    // Use the first letter of the first 2 segments
    let p0 = segments[0].chars().nth(0)?;
    let p1 = segments[1].chars().nth(0)?;

    // Get chipset name
    let chipset = segments[2];

    // Use the first letters from the next 2 segments
    let p2 = segments[3].chars().nth(0)?;
    let p3 = segments[4].chars().nth(0)?;

    let output = format!("{0}{1}{2}{3}{4}", p0, p1, chipset, p2, p3);

    Some(output)
}

fn success_msg(new_path: &String, old_path: &String) -> String {
    let mut msg = format!("{0} ===> {1}\n\n", old_path, new_path);

    msg = format!(
        "{0}This file has been renamed to {1}. To use USB BIOS Flashback, \
        copy {1} to the root of your USB flash drive.", msg, new_path
    );

    return msg;
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Error: input file required.");
        exit(1);
    }

    let file_path = Path::new(args[1].as_str());
    let ext = file_path.extension()
        .unwrap_or(OsStr::new("CAP")).to_str().unwrap();
    let file_name = String::from(file_path.file_stem().unwrap().to_str().unwrap());
    let parent_dir = file_path.parent()
        .unwrap_or(Path::new("./"));
    let new_name = format!("{}.{}", rename_file(&file_name).unwrap(), ext );
    let new_path = parent_dir.join(&new_name);

    // Move the file
    fs::rename(&file_path, &new_path)?;

    let old_path_str = String::from(file_path.to_str().unwrap_or(&*file_name));

    let new_path_str = String::from(new_path.to_str().unwrap_or(&*new_name));

    println!("{}", success_msg(&new_path_str, &old_path_str));

    Ok(())
}
