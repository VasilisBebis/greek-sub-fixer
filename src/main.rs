use std::env;
use std::fs::{self, File};
use encoding::{Encoding, DecoderTrap};
use encoding::all::ISO_8859_7;
use std::io::{Write, Result};
use std::path::{PathBuf, Path};
use std::ffi::OsStr;

fn fix_sub_file(sub_file: &Path) -> Result<()> {
    let subs: Vec<u8> = fs::read(sub_file).unwrap();

    let utf8_subs = String::from(ISO_8859_7.decode(&subs, DecoderTrap::Replace).unwrap());

    let utf8_subs = utf8_subs.replace("\u{2019}", "\u{0386}");
    let mut file = File::create(sub_file)?;
    file.write_all(utf8_subs.as_bytes())?;

    Ok(())
}

fn main() -> Result<()>{
    let mut arguments = env::args_os();
    //TODO: do better error detection
    assert!(arguments.len() == 2); // TODO: temporary
    arguments.next(); // skip program name
    let path: PathBuf = PathBuf::from(arguments.next().unwrap());
    if !path.is_dir() {
        panic!("invalid directory provided");
    }

    for entry in path.read_dir().expect("reading failed") {
        if let Ok(entry) = entry {
            if let Some(extension) = entry.path().extension() {
                if extension == "srt" {
                    let result = fix_sub_file(&entry.path());
                    if result.is_err() {
                        panic!("error fixing file");
                    }
                }
            }
        }
    }
    Ok(())
}
