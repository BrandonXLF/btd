use std::error::Error;

pub fn show_help() -> Result<(), Box<dyn Error>> {
    println!("Usage: btd [<name>]
       btd <--create | --delete | --edit | --rename> [<name>] [--dir <dir>] [--base <base>]
       btd <--set-dir | --set-base> [<value>]
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
    --dir [<dir>]   Read instruction files from the library located in the <dir> directory. Pass
                    \"--lib base\" to use the default library location.
    --base [<base>] Base directory for the meta dir of instruction files. Defaults to the current
                    working directory.

Library default config:
    --set-dir [<value>]     Set the default directory to use as the library's location.
    --set-base [<value>]    Set the default base to use for the meta directory of instruction files.

Program information:
    --help          Show this help message and exit.
    --version       Print the version of btd and exit.

Instruction file format:
    Visit https://brandonxlf.github.io/btd/file-format/ for information in the instruction file file
    format.");

    Ok(())
}

pub fn show_version() -> Result<(), Box<dyn Error>> {
    println!("{} {}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));
    Ok(())
}
