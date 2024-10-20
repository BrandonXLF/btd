use std::{error::Error, fs::File, path::PathBuf};

use crate::{args::Args, transformation::Transformation};

pub fn read_instruction_file(name: &PathBuf) -> Result<Vec<Transformation>, Box<dyn Error>> {
    let reader = File::open(name)?;
    let steps: Vec<Transformation> = serde_yaml::from_reader(reader)?;
    return Ok(steps);
}

fn resolve_path(path: &str) -> Option<PathBuf> {
    let mut path = PathBuf::from(path);

    if path.is_file() {
        return Some(path);
    }

    if !path.extension().map_or(false, |ext| ext == "yml") {
        path.set_extension("yml");

        if path.is_file() {
            return Some(path);
        }
    }

    None
}

fn resolve_name_or_path(name_or_path: &str, args: &Args) -> Result<PathBuf, Box<dyn Error>> {
    if let Some(path) = resolve_path(name_or_path) {
        Ok(path)
    } else if let Some(path) = args.get_lib()?.resolve_name(name_or_path) {
        Ok(path)
    } else {
        return Err(
            "No Instruction File found. Run btd --list to see available Instruction Files.".into(),
        );
    }
}

pub fn find_instructions(
    name_or_path: Option<&str>,
    args: &Args
) -> Result<Vec<Transformation>, Box<dyn Error>> {
    if let Some(name_or_path) = name_or_path {
        let path = resolve_name_or_path(name_or_path, args)?;
        read_instruction_file(&path)
    } else {
        Ok(args.get_lib()?.match_by_dir()?.1)
    }
}
