

use std::env;
use std::fs;
use std::path::Path;
use std::ffi::OsStr;

fn rename_file(file_name: &String) -> Option<String> {
    //! Takes the original file name of the form "{}-{}-\<chipset\>-{}-{}-ASUS-\<version\>" -- where '{}'
    //! represents a word in the product name -- and converts it to a form that the BIOS will
    //! recognize.

    let segments: Vec<&str> = file_name.split("-").collect();
    // Use the first letter of the first 2 segments
    // let mut output = get_first_char(segments[0]) + &*get_first_char(segments[1]);
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
    // if args.len() < 2 {
    //     return Err("Error: file name argument missing.");
    // }

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
