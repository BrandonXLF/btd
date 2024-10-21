use std::{env, error::Error, path::PathBuf};

use crate::{builder::Builder, library::Library, out_of_lib::OutOfLibrary};

pub struct Args {
    lib: Option<String>,
    base_path: Option<PathBuf>,
    other_args: Vec<String>
}

impl Args {
    pub fn new() -> Args {
        let mut next_lib = false;
        let mut lib = None;

        let mut next_base_path = false;
        let mut base_path = None;

        let mut other_args: Vec<String> = Vec::new();

        for arg in env::args() {
            if next_lib {
                lib = Some(arg);
                next_lib = false;
            } else if next_base_path {
                base_path = Some(PathBuf::from(arg));
                next_base_path = false;
            } else if arg == "--lib" {
                next_lib = true;
            } else if arg == "--base" {
                next_base_path = true;
            } else {
                other_args.push(arg);
            }
        }

        return Args{ lib, base_path, other_args };
    }

    pub fn get_lib(&self) -> Result<Library, Box<dyn Error>> {
        Library::new(self.lib.as_deref(), self.base_path.as_deref())
    }

    pub fn get_out_of_lib(&self) -> OutOfLibrary {
        OutOfLibrary { base: self.base_path.clone() }
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
            "--set-lib" => self.get_lib()?.save_config("link", self.get_unnamed()),
            "--set-base" => self.get_lib()?.save_config("base", self.get_unnamed()),
            "--help" => show_help(),
            "--version" => show_version(),
            _ => Builder::process_file(Some(&self.other_args[1]), &self),
        }
    }
}

pub fn show_help() -> Result<(), Box<dyn Error>> {
    println!("Usage: btd [<name>]
       btd <--create | --delete | --edit | --rename> [<name>] [--lib <lib>] [--base <base>]
       btd <--set-lib | --set-base> [<value>]
       btd <--list | --open>
       btd <--help | --version>

Running instruction files:
    [<name>]    If no <name> is specified, the script in the library with its meta dir set to the
                current directory will be run. Otherwise, if <name> is a path that exists, it will
                be run without any library defaults. If it isn't, the script with the name <name> in
                the library will be run. .yml is added to <name> as required.

Library actions:
    --create [<name>]   Create a new instruction file in the library with the given name. If no name
                        is given, a prompt will show to enter a name.
    --delete [<name>]   Delete the instruction file in the library with the give name. If no name is
                        given, the instruction file corresponding to the current directory will be
                        deleted.
    --edit [<name>]     Edit the instruction file in the library with the given name. If no name is
                        given, the instruction file corresponding to the current directory will be
                        opened for editing.
    --list              List all instruction files in the library.
    --open              Open the directory containing the library.
    --rename [<name>]   Rename the instruction file in the library with the given name. If no name is
                        given, the instruction file corresponding to the current directory will be
                        renamed.

Library options:
    --lib [<lib>]   Read instruction files from the library located in the <lib> directory. Pass
                    \"--lib base\" to use the default library location.
    --base [<base>] Base directory for the meta dir of instruction files. Defaults to the current
                    working directory.

Library default config:
    --set-lib [<value>]     Set the default directory to use as the library's location.
    --set-base [<value>]    Set the default base to use for the meta directory of instruction files.

Program information:
    --help          Show this help message and exit.
    --version       Print the version of btd and exit.

Instruction file format:
    Visit https://brandonxlf.github.io/btd/file-format/ for information in the instruction file file
    format.");

    Ok(())
}

fn show_version() -> Result<(), Box<dyn Error>> {
    println!("{} {}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));
    Ok(())
}
