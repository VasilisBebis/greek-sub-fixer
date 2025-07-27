use std::fs::{self, File};
use encoding::DecoderTrap;
use std::io::Write;
use std::path::Path;
use chardet;


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

pub fn parse_cli(exec_string: &mut std::env::ArgsOs) -> Result<(), String> {
    if exec_string.len() == 3 {
        match exec_string.nth(1).unwrap().to_str() {
            Some("-d") | Some("--dir") => {
                println!("{:?}", exec_string.next().unwrap());
                todo!("Directory Handling");
            },
            Some("-f") | Some("--file") => {
                let result = fix_sub_file(exec_string.next().unwrap().as_ref());
                if result.is_err() {
                    Err(String::from(format!("{RED_BOLD}ERROR{RESET_ANSI_STYLE}: Failed to fix sub file")))
                } else {
                    Ok(())
                }
            }, 
            _ => {
                Err(String::from(format!("{RED_BOLD}ERROR{RESET_ANSI_STYLE}: invalid operation")))
            },
        }

    } else if exec_string.len() == 2 {
        match exec_string.nth(1).unwrap().to_str() {
            Some("-h") | Some("--help") => {
                print_help();
                Ok(())
            },
            _ => {
                Err(String::from(format!("{RED_BOLD}ERROR{RESET_ANSI_STYLE}: invalid operation")))
            },
        }
    } else {
        Err(String::from(format!("{RED_BOLD}ERROR{RESET_ANSI_STYLE}: invalid number of arguments provided")))
    }
}

fn fix_sub_file(sub_file: &Path) -> Result<(), ()> {
    let subs: Vec<u8> = fs::read(sub_file)
                        .map_err(|err| 
                                eprintln!("Could not open file {sub_file:?} because: {err}")
                                )?;

    let result = chardet::detect(&subs);
    let detected_charset = &result.0;
    if &result.0 == "UTF-8" {
        Ok(())
    } else {
        let coder = encoding::label::encoding_from_whatwg_label(chardet::charset2encoding(detected_charset));
        if coder.is_some() {
            let mut utf8_subs = coder.unwrap().decode(&subs, DecoderTrap::Ignore)
                .map_err(|err| eprintln!("Decoding failed because of: {err}"))?;
            utf8_subs = utf8_subs.replace("\u{2019}", "\u{0386}");
            let mut file = File::create(sub_file)
                .map_err(|err| eprintln!("Could not open file {sub_file:?} because: {err}"))?;
            file.write_all(utf8_subs.as_bytes())
                .map_err(|err| eprintln!("Could not write to file because: {err}"))?;
            Ok(())
        } else {
            eprintln!("Encoding detection failed");
            Err(())
        }
    }
}

