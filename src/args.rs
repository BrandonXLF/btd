use std::{env, error::Error, path::PathBuf};

use crate::{builder::Builder, library::Library, messages::{show_help, show_version}, out_of_lib::OutOfLibrary};

pub struct Args {
    dir: Option<String>,
    base_path: Option<PathBuf>,
    other_args: Vec<String>
}

impl Args {
    pub fn new() -> Args {
        let mut next_dir = false;
        let mut dir = None;

        let mut next_base_path = false;
        let mut base_path = None;

        let mut other_args: Vec<String> = Vec::new();

        for arg in env::args() {
            if next_dir {
                dir = Some(arg);
                next_dir = false;
            } else if next_base_path {
                base_path = Some(PathBuf::from(arg));
                next_base_path = false;
            } else if arg == "--dir" {
                next_dir = true;
            } else if arg == "--base" {
                next_base_path = true;
            } else {
                other_args.push(arg);
            }
        }

        return Args{ dir, base_path, other_args };
    }

    pub fn get_lib(&self) -> Result<Library, Box<dyn Error>> {
        Library::new(self.dir.as_deref(), self.base_path.as_deref())
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
            "--set-dir" => self.get_lib()?.save_config("link", self.get_unnamed()),
            "--set-base" => self.get_lib()?.save_config("base", self.get_unnamed()),
            "--help" => show_help(),
            "--version" => show_version(),
            _ => Builder::process_file(Some(&self.other_args[1]), &self),
        }
    }
}
