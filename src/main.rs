mod args;
mod builder;
mod instruction_file;
mod library;
mod library_config;
mod messages;
mod out_of_lib;
mod transformation;

use std::process::exit;

use args::Args;

fn main() {
    let args = Args::new();
    let res = args.process();

    match res {
        Ok(_) => {}
        Err(err) => {
            eprintln!("\nError: {}", err);
            exit(1);
        }
    }
}
