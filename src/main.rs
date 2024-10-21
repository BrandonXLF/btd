mod args;
mod builder;
mod library;
mod instruction_file;
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
