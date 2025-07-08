use std::env;
use std::fs::{self, File};
use encoding::{Encoding, DecoderTrap};
use encoding::all::ISO_8859_7;
use std::io::{Write, Result};

fn main() -> Result<()>{
    let mut arguments = env::args_os();
    //TODO: do better error detection
    assert!(arguments.len() == 2); // TODO: temporary
    arguments.next();
    let binding = arguments.next().expect("sub file provided");
    let sub_file = binding.to_str().unwrap();

    let subs: Vec<u8> = fs::read(sub_file).unwrap();

    let utf8_subs = String::from(ISO_8859_7.decode(&subs, DecoderTrap::Replace).unwrap());

    let utf8_subs = utf8_subs.replace("\u{2019}", "\u{0386}");
    let mut file = File::create(sub_file)?;
    file.write_all(utf8_subs.as_bytes())?;

    Ok(())
}
