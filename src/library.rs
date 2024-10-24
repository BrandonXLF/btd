use std::{
    env,
    error::Error,
    fs::{self, read_dir, remove_file, rename, File},
    io::{stdin, stdout, Write},
    path::{Path, PathBuf},
};

use crate::{
    instruction_file::{read, InstructionFile},
    library_config::LibraryConfig,
    transformation::MetaTransformation,
};

pub struct Library {
    base_dir: PathBuf,
    dir: PathBuf,
    base: Option<PathBuf>,
}

impl Library {
    pub fn new(path_str: Option<&str>, base_str: Option<&Path>) -> Result<Library, Box<dyn Error>> {
        let mut base_dir = dirs::data_dir().ok_or_else(|| "Failed to get config directory")?;
        base_dir.push("btd-library");
        fs::create_dir_all(&base_dir).map_err(|_| "Could not make base library directory")?;

        let dir = match path_str {
            Some("base") => base_dir.clone(),
            Some(path) => PathBuf::from(path),
            None => base_dir.clone(),
        };
        fs::create_dir_all(&dir).map_err(|_| "Could not make library directory")?;

        let base: Option<PathBuf> = base_str.map(|x| x.to_path_buf());

        let mut lib = Library {
            base_dir,
            dir,
            base,
        };

        lib.apply_saved_config(path_str.is_some());

        return Ok(lib);
    }

    fn apply_link_dir(&mut self, path_str: &str) -> Result<(), Box<dyn Error>> {
        let path = PathBuf::from(&path_str);

        match fs::create_dir_all(&path) {
            Ok(_) => {
                self.dir = path;
                Ok(())
            }
            Err(_) => Err("Could not make linked library directory".into()),
        }
    }

    fn name_to_path(&self, name: &str) -> PathBuf {
        let mut path = self.dir.join(name);

        if !path.extension().map_or(false, |ext| ext == "yml") {
            path.set_extension("yml");
        }

        path
    }

    pub fn read_name(&self, name: &str) -> Result<Option<InstructionFile>, Box<dyn Error>> {
        let path = self.name_to_path(name);

        if path.is_file() {
            return Ok(Some(read(&path, self.base.as_deref())?));
        }

        Ok(None)
    }

    pub fn get_all_files(&self) -> Result<impl Iterator<Item = PathBuf>, Box<dyn Error>> {
        return Ok(read_dir(&self.dir)
            .map_err(|_| "Failed to read the Library directory")?
            .filter_map(Result::ok)
            .filter(|entry| entry.path().is_file())
            .map(|entry| entry.path()));
    }

    fn get_existing_dir(&self, name: Option<&str>) -> Result<PathBuf, Box<dyn Error>> {
        let path = if let Some(name) = name {
            self.name_to_path(name)
        } else {
            self.match_cwd()?.0
        };

        if !path.is_file() {
            return Err("Instruction file not found".into());
        }

        Ok(path)
    }

    pub fn list_files(&self) -> Result<(), Box<dyn Error>> {
        for file in self.get_all_files()? {
            if let Ok(inst) = read(&file, self.base.as_deref()) {
                println!(
                    "{} - {}",
                    file.file_stem().unwrap().to_string_lossy(),
                    inst.dir.to_string_lossy()
                );
            }
        }

        Ok(())
    }

    pub fn create_file(&self, name: Option<&str>) -> Result<(), Box<dyn Error>> {
        let path = if let Some(name) = name {
            self.name_to_path(name)
        } else {
            print!("\nEnter instruction file name: ");
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

        let actual_name = match name {
            Some(name) => name.to_owned(),
            None => path
                .with_extension("")
                .file_name()
                .ok_or("Unknown file name")?
                .to_string_lossy()
                .into_owned(),
        };

        print!("\nAre you sure you want to delete {}? (Y/N): ", actual_name);
        let _ = stdout().flush();

        let mut res = String::new();
        stdin().read_line(&mut res)?;

        let trimmed = res.trim();

        if !(trimmed == "Y" || trimmed == "y") {
            return Err("Deletion aborted".into());
        }

        remove_file(path).map_err(|_| "Failed to remove instruction file")?;
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

    pub fn match_cwd(&self) -> Result<(PathBuf, InstructionFile), Box<dyn Error>> {
        let cwd = env::current_dir()?;

        for path in self.get_all_files()? {
            if let Ok(inst) = read(&path, self.base.as_deref()) {
                if inst.dir == cwd {
                    return Ok((path, inst));
                }
            }
        }

        return Err(
            "No instruction files match the current directory. Run btd --help for help.".into(),
        );
    }

    fn get_config_file_path(&self) -> PathBuf {
        self.base_dir.join(".config")
    }

    fn read_saved_config(&self) -> Result<Option<LibraryConfig>, Box<dyn Error>> {
        match File::open(self.get_config_file_path()) {
            Ok(config_file) => {
                let config_data: LibraryConfig = serde_yaml::from_reader(config_file)?;
                Ok(Some(config_data))
            }
            Err(_) => Ok(None),
        }
    }

    fn apply_config_value(&mut self, name: &str, val: &str) -> Result<(), Box<dyn Error>> {
        match name {
            "dir" => self.apply_link_dir(val)?,
            "base" => self.base = Some(PathBuf::from(val)),
            &_ => return Err(format!("Unknown Library config {}", name).into()),
        }

        Ok(())
    }

    fn reset_config_value(&mut self, name: &str) -> Result<(), Box<dyn Error>> {
        match name {
            "dir" => self.dir = self.base_dir.clone(),
            "base" => self.base = None,
            &_ => return Err(format!("Unknown Library config {}", name).into()),
        }

        Ok(())
    }

    pub fn save_config_value(
        &mut self,
        name: &str,
        val: Option<&str>,
    ) -> Result<(), Box<dyn Error>> {
        match val {
            Some(val) => self.apply_config_value(name, val)?,
            None => self.reset_config_value(name)?,
        }

        let mut config = self
            .read_saved_config()
            .unwrap_or(None)
            .unwrap_or_else(|| LibraryConfig::new_empty());

        match name {
            "dir" => config.dir = val.map(|x| x.to_owned()),
            "base" => config.base = val.map(|x| x.to_owned()),
            &_ => return Err(format!("Unknown Library config {}", name).into()),
        }

        let file = File::create(self.get_config_file_path())
            .map_err(|_| "Could not open library config for writing".to_owned())?;

        serde_yaml::to_writer(file, &config).map_err(|_| "Could not write to library config".into())
    }

    fn apply_saved_config_value(&mut self, name: &str, val: Option<&str>) -> () {
        if let Some(val) = val {
            match self.apply_config_value(name, &val) {
                Ok(_) => (),
                Err(err) => eprintln!("{}", err),
            }
        }
    }

    fn apply_saved_config(&mut self, dir_was_set: bool) -> () {
        match self.read_saved_config() {
            Ok(maybe_config) => {
                if let Some(config) = maybe_config {
                    if !dir_was_set {
                        self.apply_saved_config_value("dir", config.dir.as_deref());
                    }

                    if self.base.is_none() {
                        self.apply_saved_config_value("base", config.base.as_deref());
                    }
                }
            }
            Err(_) => {
                eprintln!("Failed to read library config");
            }
        }
    }
}
