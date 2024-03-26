use std::{env, error::Error};

use crate::{chef::Chef, library::Library};

pub fn process_args(args: Vec<String>) -> Result<(), Box<dyn Error>> {
    if args.len() <= 1 {
        return Chef::process_recipe(None);
    }

    match args[1].as_str() {
        "--create" => Library::new()?.create_recipe(args.get(2).map(|x| x.as_str())),
        "--delete" => Library::new()?.delete_recipe(args.get(2).map(|x| x.as_str())),
        "--edit" => Library::new()?.edit_recipe(args.get(2).map(|x| x.as_str())),
        "--list" => Library::new()?.list_recipes(),
        "--open" => Library::new()?.open(),
        "--rename" => Library::new()?.rename_recipe(args.get(2).map(|x| x.as_str())),
        "--help" => show_help(),
        "--version" => show_version(),
        _ => Chef::process_recipe(Some(&args[1])),
    }
}

pub fn show_help() -> Result<(), Box<dyn Error>> {
    println!("Usage: tdep [<name>]
       tdep <--create | --delete | --edit | --rename> [<name>]
       tdep <--list | --open>
       tdep <--help | --version>

Running Instruction Files
    [<name>]    If no <name> is specified, the script in The Library with its meta dir set to the
                current directory will be run. Otherwise, the <name> script will be run if it exists,
                and if it doesn't, the script with the name <name> in The Library will be run. .yml
                is added to <name> as required.

The Library
    --create [<name>]   Create a new recipe in The Library with the given name. If no name is
                        given, a prompt will show to enter a name.
    --delete [<name>]   Delete the recipe in The Library with the give name. If no name is given,
                        the recipe corresponding to the current directory will be deleted.
    --edit [<name>]     Edit the recipe in The Library with the given name. If no name is given,
                        the recipe corresponding to the current directory will be opened for editing.
    --list              List all Instruction Files in The Library.
    --open              Open the directory containing The Library.
    --rename [<name>]   Rename the recipe in The Library with the given name. If no name is given,
                        the recipe corresponding to the current directory will be renamed.

Program Information
    --help          Show this help message and exit.
    --version       Print the version of tdep and exit.

Instruction File Format
    Visit https://brandonxlf.github.io/tdep/docs/file-format for information in the Instruction File
    file format.");

    Ok(())
}

fn show_version() -> Result<(), Box<dyn Error>> {
    println!("{} {}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));
    Ok(())
}
