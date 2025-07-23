use std::io::{Error, ErrorKind};
use std::fs::{self, File};
use encoding::{Encoding, DecoderTrap};
use encoding::all::ISO_8859_7;
use std::io::{Write, Result};
use std::path::Path;


const YELLOW_BOLD_UNDERLINE: &str    = "\x1b[1;4;33m";
const RED_BOLD_UNDERLINE:    &str    = "\x1b[1;4;31m";
const RED_BOLD:              &str    = "\x1b[1;31m";
const RESET_ANSI_STYLE:      &str    = "\x1b[0m";

pub fn print_help() {
    let bin_name = if cfg!(windows) {
        "greek-sub-fixer.exe"
    } else {
        "greek-sub-fixer"
    };

    println!("{RED_BOLD_UNDERLINE}Usage{RESET_ANSI_STYLE}: {bin_name} [OPTION]");
    println!("");
    println!("{YELLOW_BOLD_UNDERLINE}Options{RESET_ANSI_STYLE}:");
    println!("  {:<20} Specify directory to fix all contained .srt files.", "-d, --dir <path>");
    println!("  {:<20} Specify .srt file to fix.", "-f, --file <path>");
    println!("  {:<20} Show help page and exit.", "-h, --help");
}

pub fn parse_cli(exec_string: &mut std::env::ArgsOs) -> std::io::Result<()> {
    let bin_name = if cfg!(windows) {
        "greek-sub-fixer.exe"
    } else {
        "greek-sub-fixer"
    };

    if exec_string.len() == 3 {
        match exec_string.nth(1).unwrap().to_str() {
            Some("-d") | Some("--dir") => {
                println!("{:?}", exec_string.next().unwrap());
                todo!("Directory Handling");
            },
            Some("-f") | Some("--file") => {
                println!("{:?}",exec_string.next().unwrap());
                todo!("File Handling");
            }, 
            _ => {
                println!("{RED_BOLD}ERROR{RESET_ANSI_STYLE}: Invalid Operation");
                println!("Use \"{bin_name} -h\" to see help page");
            },
        }

        Ok(())
    } else if exec_string.len() == 2 {
        match exec_string.nth(1).unwrap().to_str() {
            Some("-h") | Some("--help") => print_help(),
            _ => {
                println!("{RED_BOLD}ERROR{RESET_ANSI_STYLE}: Invalid Operation");
                println!("Use \"{bin_name} -h\" to see help page");
            },
        }

        Ok(())
    } else {

        Err(Error::new(ErrorKind::Other, "invalid arguments"))
    }
}

fn fix_sub_file(sub_file: &Path) -> Result<()> {
    let subs: Vec<u8> = fs::read(sub_file).unwrap();

    let utf8_subs = String::from(ISO_8859_7.decode(&subs, DecoderTrap::Replace).unwrap());

    let utf8_subs = utf8_subs.replace("\u{2019}", "\u{0386}");
    let mut file = File::create(sub_file)?;
    file.write_all(utf8_subs.as_bytes())?;

    Ok(())
}

