use std::{env, error::Error};

use crate::{builder::Builder, library::Library};

pub struct Args {
    lib_path: Option<String>,
    other_args: Vec<String>
}

impl Args {
    pub fn new() -> Args {
        let mut other_args: Vec<String> = Vec::new();
        let mut next_lib_path = false;
        let mut lib_path = None;

        for arg in env::args() {
            if next_lib_path {
                lib_path = Some(arg);
                next_lib_path = false;
            } else if arg == "--lib" {
                next_lib_path = true;
            } else {
                other_args.push(arg);
            }
        }

        return Args{ lib_path, other_args };
    }

    pub fn get_lib(&self) -> Result<Library, Box<dyn Error>> {
        Library::new(self.lib_path.as_deref())
    }

    fn get_unnamed(&self) -> Option<&str> {
        self.other_args.get(2).map(|x| x.as_str())
    }

    pub fn process(&self) -> Result<(), Box<dyn Error>> {
        if self.other_args.len() <= 1 {
            return Builder::process_file(None, &self);
        }

        match self.other_args[1].as_str() {
            "--create" => self.get_lib()?.create_file(self.get_unnamed()),
            "--delete" => self.get_lib()?.delete_file(self.get_unnamed()),
            "--edit" => self.get_lib()?.edit_file(self.get_unnamed()),
            "--list" => self.get_lib()?.list_files(),
            "--open" => self.get_lib()?.open(),
            "--rename" => self.get_lib()?.rename_file(self.get_unnamed()),
            "--set-lib" => self.get_lib()?.write_link(self.get_unnamed()),
            "--help" => show_help(),
            "--version" => show_version(),
            _ => Builder::process_file(Some(&self.other_args[1]), &self),
        }
    }
}

pub fn show_help() -> Result<(), Box<dyn Error>> {
    println!("Usage: btd [<name>]
       btd <--create | --delete | --edit | --rename> [<name>] [--lib <lib>]
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
    --set-lib           Set the directory to use as the library location. You can access the default
                        library using \"--lib base\".

Library Options
    --lib [<lib>]   Read instruction files from the library located in the <lib> directory. Pass <lib>
                    as base to use the default library location.

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
