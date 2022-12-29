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

use chrono::NaiveDate;
use std::{
    fs::File,
    io::{BufReader, Read},
    ops::Range,
    path::Path,
};

const EXPECTED_FILE_SIZE: u64 = 33558528;

/// Byte offset from start of .CAP file where the BIOS info resides
const BIOS_INFO_START: usize = 0x10000FA;
/// Address of last BIOS info byte in the .CAP file
const BIOS_INFO_END: usize = 0x100018B;

// TODO: Finish documentation

const BOARD_NAME_START: usize = 0x00;
const BOARD_NAME_END: usize = 0x3B;

const BRAND_NAME_START: usize = 0x3C;
const BRAND_NAME_END: usize = 0x50;

const DATE_START: usize = 0x51;
const DATE_END: usize = 0x5B;

const BUILD_NUMBER_START: usize = 0x5C;
const BUILD_NUMBER_END: usize = 0x69;

const CAP_NAME_START: usize = 0x83;
const CAP_NAME_END: usize = 0x8F;

/// Information describing the
#[derive(Debug)]
pub struct BiosInfo {
    /// Name of target motherboard
    board_name: String,
    /// Brand of motherboard
    brand: String,
    /// Reported BIOS build date
    build_date: NaiveDate,
    /// Reported build number
    build_number: String,
    /// Filename the target motherboard expects this file to be named
    ///
    /// # Examples:
    ///     - "TGX570PW.CAP"
    ///     - "C8DH.CAP"
    expected_name: String,
}

/// Returns a new String where all characters after the first NULL have been removed
///
/// # Arguments
///
/// * `s` - input string
fn trim_after_null(s: &str) -> String {
    let mut trimmed = String::new();

    for ch in s.chars() {
        if ch == '\0' {
            break;
        }
        trimmed.push(ch);
    }

    trimmed
}

fn bytes_to_string(bytes: &Vec<u8>, range: Range<usize>) -> String {
    let chunk = &bytes[range];
    let tmp_str = String::from_utf8_lossy(chunk);

    trim_after_null(&tmp_str)
}

impl BiosInfo {
    pub fn from_file(bios_file: &mut File) -> Result<Self, std::io::Error> {
        // Read in raw bytes of info struct
        let mut reader = BufReader::new(bios_file);
        let read_size = BIOS_INFO_END - BIOS_INFO_START;
        let mut info_chunk = Vec::with_capacity(read_size);

        reader.seek_relative(BIOS_INFO_START as i64)?;
        reader.take(read_size as u64).read_to_end(&mut info_chunk)?;
        // println!("Read {n} bytes from input file.");

        // Read each field out of the info chunk
        let board_name = bytes_to_string(&info_chunk, BOARD_NAME_START..BOARD_NAME_END);
        // println!("Board name: {board_name}");
        let brand = trim_after_null(&String::from_utf8_lossy(
            &info_chunk[BRAND_NAME_START..BRAND_NAME_END],
        ));
        // println!("Brand: {brand}");

        let build_date =
            trim_after_null(&String::from_utf8_lossy(&info_chunk[DATE_START..DATE_END]));
        // println!("Build date: {}", build_date);
        let build_date =
            NaiveDate::parse_from_str(&build_date, "%m/%d/%Y").unwrap_or(NaiveDate::default());
        // println!("Parsed build date: {}", build_date);

        let build_num = trim_after_null(&String::from_utf8_lossy(
            &info_chunk[BUILD_NUMBER_START..BUILD_NUMBER_END],
        ));

        let cap_name = trim_after_null(&String::from_utf8_lossy(
            &info_chunk[CAP_NAME_START..CAP_NAME_END],
        ));

        Ok(BiosInfo {
            board_name: board_name.to_string(),
            brand: brand.to_string(),
            build_date: build_date,
            build_number: build_num.to_string(),
            expected_name: cap_name.to_string(),
        })
    }

    pub fn get_expected_name(&self) -> &String {
        &self.expected_name
    }
}

/// Returns `true` if the provided file meets known requirements
///
/// # Details
///
/// Currently, only the expected size of the file can be checked. It is yet to be determined if
/// these files have something like an embedded checksum and where that might be.
///
/// # Arguments
///
/// * `bios_file` - file to verify
pub fn is_file_valid(bios_file: &File) -> Result<bool, std::io::Error> {
    let file_info = bios_file.metadata()?;

    return if !file_info.is_file() {
        Ok(false)
    } else {
        Ok(file_info.len() == EXPECTED_FILE_SIZE)
    };
}
