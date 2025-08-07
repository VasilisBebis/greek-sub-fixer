use std::env;
pub mod cli_app;

fn main() {
    let mut arguments = env::args_os();

    let ret = cli_app::parse_cli(&mut arguments);
    //TODO: handle return result
    if let Err(error) = ret {
        println!("{error}");
    }
}

