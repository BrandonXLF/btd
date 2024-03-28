mod args;
mod builder;
mod library;
mod read;
mod transformation;

use std::{env, process::exit};

use args::process_args;

fn main() {
    let args: Vec<String> = env::args().collect();
    let res = process_args(args);

    match res {
        Ok(_) => {}
        Err(err) => {
            eprintln!("\nError: {}", err);
            exit(1);
        }
    }
}
