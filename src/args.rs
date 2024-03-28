use std::{env, error::Error};

use crate::{builder::Builder, library::Library};

pub fn process_args(args: Vec<String>) -> Result<(), Box<dyn Error>> {
    if args.len() <= 1 {
        return Builder::process_file(None);
    }

    match args[1].as_str() {
        "--create" => Library::new()?.create_file(args.get(2).map(|x| x.as_str())),
        "--delete" => Library::new()?.delete_file(args.get(2).map(|x| x.as_str())),
        "--edit" => Library::new()?.edit_file(args.get(2).map(|x| x.as_str())),
        "--list" => Library::new()?.list_files(),
        "--open" => Library::new()?.open(),
        "--rename" => Library::new()?.rename_file(args.get(2).map(|x| x.as_str())),
        "--help" => show_help(),
        "--version" => show_version(),
        _ => Builder::process_file(Some(&args[1])),
    }
}

pub fn show_help() -> Result<(), Box<dyn Error>> {
    println!("Usage: btd [<name>]
       btd <--create | --delete | --edit | --rename> [<name>]
       btd <--list | --open>
       btd <--help | --version>

Running Instruction Files
    [<name>]    If no <name> is specified, the script in The Library with its meta dir set to the
                current directory will be run. Otherwise, the <name> script will be run if it exists,
                and if it doesn't, the script with the name <name> in The Library will be run. .yml
                is added to <name> as required.

The Library
    --create [<name>]   Create a new Instruction File in The Library with the given name. If no name
                        is given, a prompt will show to enter a name.
    --delete [<name>]   Delete the Instruction File in The Library with the give name. If no name is
                        given, the Instruction File corresponding to the current directory will be
                        deleted.
    --edit [<name>]     Edit the Instruction File in The Library with the given name. If no name is
                        given, the Instruction File corresponding to the current directory will be
                        opened for editing.
    --list              List all Instruction Files in The Library.
    --open              Open the directory containing The Library.
    --rename [<name>]   Rename the Instruction File in The Library with the given name. If no name is
                        given, the Instruction File corresponding to the current directory will be
                        renamed.

Program Information
    --help          Show this help message and exit.
    --version       Print the version of btd and exit.

Instruction File Format
    Visit https://brandonxlf.github.io/btd/file-format/ for information in the Instruction File
    file format.");

    Ok(())
}

fn show_version() -> Result<(), Box<dyn Error>> {
    println!("{} {}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));
    Ok(())
}
