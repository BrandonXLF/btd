use std::{error::Error, fs::File, path::PathBuf};

use crate::{args::Args, transformation::{Transformation, TransformationTrait}};

pub struct InstructionFile {
    pub dir: PathBuf,
    pub steps: Vec<Transformation>
}

pub fn read(path: &PathBuf) -> Result<InstructionFile, Box<dyn Error>> {
    let reader = File::open(path)?;
    let mut steps: Vec<Transformation> = serde_yaml::from_reader(reader)?;

    if steps.len() == 0 {
        return Err("Missing meta step".into());
    }
    
    let meta = steps.remove(0);
    let dir = PathBuf::from(meta.get_req_str("dir", 0)?);
    
    return Ok(InstructionFile { dir, steps });
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

pub fn find(name_or_path: Option<&str>, args: &Args) -> Result<InstructionFile, Box<dyn Error>> {
    if let Some(name_or_path) = name_or_path {
        let path = resolve_name_or_path(name_or_path, args)?;
        read(&path)
    } else {
        Ok(args.get_lib()?.match_cwd()?.1)
    }
}
