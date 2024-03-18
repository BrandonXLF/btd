use std::{error::Error, fs::File, path::PathBuf};

use crate::{library::Library, transformation::Transformation};

pub fn read_recipe_file(name: &PathBuf) -> Result<Vec<Transformation>, Box<dyn Error>> {
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

fn resolve_name_or_path(name_or_path: &str) -> Result<PathBuf, Box<dyn Error>> {
    if let Some(path) = resolve_path(name_or_path) {
        Ok(path)
    } else if let Some(path) = Library::new()?.resolve_name(name_or_path) {
        Ok(path)
    } else {
        return Err("No recipe found. Run tdep --list to see available recipes.".into());
    }
}

pub fn read_recipe(name_or_path: Option<&str>) -> Result<Vec<Transformation>, Box<dyn Error>> {
    if let Some(name_or_path) = name_or_path {
        let path = resolve_name_or_path(name_or_path)?;
        read_recipe_file(&path)
    } else {
        Ok(Library::new()?.match_by_dir()?.1)
    }
}
