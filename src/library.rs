use std::{
    env,
    error::Error,
    fs::{self, read_dir, remove_file, rename, File},
    io::{stdin, stdout, Write},
    path::PathBuf,
};

use crate::{
    read::read_instruction_file,
    transformation::{MetaTransformation, Transformation, TransformationTrait},
};

pub struct Library {
    dir: PathBuf,
}

impl Library {
    pub fn new() -> Result<Library, Box<dyn Error>> {
        let mut dir = dirs::data_dir().ok_or_else(|| "Failed to get config directory")?;
        dir.push("btd-library");

        fs::create_dir_all(&dir).map_err(|_| "Could not make library directory")?;

        return Ok(Library { dir });
    }

    fn name_to_path(&self, name: &str) -> PathBuf {
        let mut path = self.dir.join(name);

        if !path.extension().map_or(false, |ext| ext == "yml") {
            path.set_extension("yml");
        }

        path
    }

    pub fn resolve_name(&self, name: &str) -> Option<PathBuf> {
        let path = self.name_to_path(name);

        if path.is_file() {
            return Some(path);
        }

        None
    }

    pub fn get_all_files(&self) -> Result<impl Iterator<Item = PathBuf>, Box<dyn Error>> {
        return Ok(read_dir(&self.dir)
            .map_err(|_| "Failed to read the library directory")?
            .filter_map(Result::ok)
            .filter(|entry| entry.path().is_file())
            .map(|entry| entry.path()));
    }

    fn get_existing_dir(&self, name: Option<&str>) -> Result<PathBuf, Box<dyn Error>> {
        let path = if let Some(name) = name {
            self.name_to_path(name)
        } else {
            self.match_by_dir()?.0
        };

        if !path.is_file() {
            return Err("Instruction File not found".into());
        }

        Ok(path)
    }
    pub fn list_files(&self) -> Result<(), Box<dyn Error>> {
        for file in self.get_all_files()? {
            if let Ok(steps) = read_instruction_file(&file) {
                let meta = steps.get(0).ok_or("Missing meta step")?;
                let dir = meta.get_req_str("dir", 0)?;

                println!("{} - {}", file.file_stem().unwrap().to_string_lossy(), dir);
            }
        }

        Ok(())
    }

    pub fn create_file(&self, name: Option<&str>) -> Result<(), Box<dyn Error>> {
        let path = if let Some(name) = name {
            self.name_to_path(name)
        } else {
            print!("\nEnter Instruction File name: ");
            let _ = stdout().flush();

            let mut name = String::new();
            stdin().read_line(&mut name)?;

            self.name_to_path(name.trim_end())
        };

        if path.is_file() {
            return Err("File already exists".into());
        }

        let steps = vec![MetaTransformation::new(
            env::current_dir()?.to_string_lossy().to_string(),
        )];

        let str = serde_yaml::to_string::<Vec<MetaTransformation>>(&steps)?;
        let res = edit::edit(str)?;

        File::create(path)
            .map_err(|_| "Failed to create file")?
            .write(res.as_bytes())
            .map_err(|_| "Failed to write to file")?;

        Ok(())
    }

    pub fn delete_file(&self, name: Option<&str>) -> Result<(), Box<dyn Error>> {
        let path = self.get_existing_dir(name)?;

        remove_file(path).map_err(|_| "Failed to remove Instruction File")?;
        Ok(())
    }

    pub fn edit_file(&self, name: Option<&str>) -> Result<(), Box<dyn Error>> {
        let path = self.get_existing_dir(name)?;

        edit::edit_file(path)?;
        Ok(())
    }

    pub fn rename_file(&self, name: Option<&str>) -> Result<(), Box<dyn Error>> {
        let old_path = self.get_existing_dir(name)?;

        print!("\nEnter new name: ");
        let _ = stdout().flush();

        let mut new_name = String::new();
        stdin().read_line(&mut new_name)?;

        let new_path = self.name_to_path(new_name.trim_end());

        rename(old_path, new_path).map_err(|_| "Failed to rename file")?;
        Ok(())
    }

    pub fn open(&self) -> Result<(), Box<dyn Error>> {
        Ok(open::that(&self.dir)?)
    }

    pub fn match_by_dir(&self) -> Result<(PathBuf, Vec<Transformation>), Box<dyn Error>> {
        let cwd = env::current_dir()?;

        for path in self.get_all_files()? {
            if let Ok(steps) = read_instruction_file(&path) {
                let step = steps.get(0);
                let meta = step.as_ref().ok_or("Missing meta step")?;
                let dir = meta.get_req_str("dir", 0)?;

                if dir == &cwd.to_string_lossy() {
                    return Ok((path, steps));
                }
            }
        }

        return Err("No Instruction Files match the current directory. Run btd --help for help.".into());
    }
}
