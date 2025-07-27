use std::env;
pub mod cli_app;

fn main() {
    let mut arguments = env::args_os();

    let ret = cli_app::parse_cli(&mut arguments);
    //TODO: handle return result
    if let Err(error) = ret {
        println!("{error}");
    }
    /* TODO: move some of this logic to cli_app.rs
    let path: PathBuf = PathBuf::from(arguments.nth(1).unwrap());
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
    */
}

