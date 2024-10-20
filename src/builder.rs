use std::{
    error::Error,
    fs,
    io::Write,
    path::{Path, PathBuf},
    process::Command,
    time::{SystemTime, UNIX_EPOCH},
};

use system::{system_output, System};

use crate::{
    args::Args, read::find_instructions, transformation::{Transformation, TransformationTrait}
};

static STAGES: &[char] = &['🥚', '🐣', '🐤', '🐔'];

pub struct Builder<'a> {
    dir: &'a Path,
    stage: usize
}

impl Builder<'_> {
    fn new(dir: &str) -> Builder {
        Builder {
            dir: Path::new(dir),
            stage: 0,
        }
    }

    pub fn process_file(name: Option<&str>, args: &Args) -> Result<(), Box<dyn Error>> {
        let mut steps = find_instructions(name, args)?;

        if steps.len() == 0 {
            return Err("Missing meta step".into());
        }

        let meta = steps.remove(0);
        let dir = meta.get_req_str("dir", 0)?;
        let mut output = Builder::new(&dir);

        for (i, step) in steps.iter().enumerate() {
            output.do_step(step, i + 1)?;
        }

        println!("\n🦖 \x1b[38;5;39mdone\x1b[0m");
        Ok(())
    }

    fn show_status(&mut self, step_type: &str, param: &str) {
        let emoji = STAGES[self.stage];

        self.stage = (self.stage + 1) % STAGES.len();

        println!(
            "\n{} \x1b[38;5;39m{}\x1b[0m \x1b[38;5;248m{}\x1b[0m",
            emoji, step_type, param
        );
    }

    fn parse_scp_path(path: &str) -> (Option<&str>, &str) {
        match path.split_once(":") {
            Some((host, file)) => {
                if host.len() > 1 {
                    (Some(host), file)
                } else {
                    (None, path)
                }
            }
            None => (None, path),
        }
    }

    fn run_remote_command(host: Option<&str>, cmd: &str) -> Result<Option<()>, Box<dyn Error>> {
        let output = match host {
            Some(host) => Command::new("ssh")
                .args(&["-tq", host, &format!("({}) && echo OK", cmd)])
                .output(),
            None => system_output(cmd),
        }
        .map_err(|_| "Failed to execute pre-deployment command")?;

        let stdout = String::from_utf8_lossy(&output.stdout);

        if stdout.trim_end() == "OK" {
            Ok(Some(()))
        } else {
            Ok(None)
        }
    }

    fn deploy_scp(from: PathBuf, to: &str, display_from: &str) -> Result<(), Box<dyn Error>> {
        println!();

        let success = Command::new("scp")
            .args(&["-r", &from.to_string_lossy(), &to])
            .status()
            .map_err(|_| "Failed to execute scp")?
            .success();

        if !success {
            return Err(format!("Failed to transfer {} to {}", display_from, to).into());
        }

        return Ok(());
    }

    fn do_step(&mut self, step: &Transformation, i: usize) -> Result<(), Box<dyn Error>> {
        let step_type = step.get_req_str("type", i)?;

        match step_type {
            "run" => {
                let cmd = step.get_req_str("cmd", i)?;
                self.show_status(&step_type, &cmd);

                let mut rust_cmd = Command::system(&cmd);

                if let Some(cwd) = step.get_opt_str("cwd", i)? {
                    rust_cmd.current_dir(self.dir.join(cwd));
                } else {
                    rust_cmd.current_dir(self.dir);
                }

                if let Some(envs) = step.get_opt_map("env", i)? {
                    for (name, val) in envs.iter() {
                        let name_str = name.as_str().ok_or_else(|| -> Box<dyn Error> {
                            "Environment variable key must be a string".into()
                        })?;

                        let val_str = val.as_str().ok_or_else(|| -> Box<dyn Error> {
                            "Environment variable value must be a string".into()
                        })?;

                        rust_cmd.env(name_str, val_str);
                    }
                }

                let success = rust_cmd
                    .status()
                    .map_err(|_| format!("Failed to run command {}", cmd))?
                    .success();

                if !success {
                    return Err(format!("Failed to run command {}", cmd).into());
                }
            }
            "create" => {
                let name = step.get_req_str("file", i)?;
                self.show_status(&step_type, &name);

                let mut file =
                    fs::File::create(self.dir.join(&name)).map_err(|_| "Unable to create file")?;

                file.write_all(step.get_req_str("text", i)?.as_bytes())
                    .map_err(|_| "Unable to write data")?;
            }
            "replace" => {
                let name = step.get_req_str("file", i)?;
                self.show_status(&step_type, &name);

                let path = self.dir.join(&name);
                let mut content = fs::read_to_string(&path).map_err(|_| "Unable to read file")?;

                let raw_str = step.get_req_str("find", i)?;
                let escaped_str: String;

                let pattern_str = if step.get_opt_in_bool("regex", i)? {
                    raw_str
                } else {
                    escaped_str = regex::escape(&raw_str);
                    &escaped_str
                };

                let pattern = regex::Regex::new(&pattern_str)?;

                content = pattern
                    .replace_all(&content, step.get_req_str("replace", i)?)
                    .to_string();

                fs::write(&path, &content).map_err(|_| "Unable to write file")?;
            }
            "prepend" => {
                let name = step.get_req_str("file", i)?;
                self.show_status(&step_type, &name);

                let path = self.dir.join(&name);
                let mut content = fs::read_to_string(&path).map_err(|_| "Unable to read file")?;

                content.insert_str(0, step.get_req_str("text", i)?);
                fs::write(&path, &content).map_err(|_| "Unable to write file")?;
            }
            "append" => {
                let name = step.get_req_str("file", i)?;
                self.show_status(&step_type, &name);

                let path = self.dir.join(&name);

                let mut file = fs::OpenOptions::new()
                    .append(true)
                    .open(&path)
                    .map_err(|_| "Unable to open file")?;

                file.write_all(step.get_req_str("text", i)?.as_bytes())
                    .map_err(|_| "Unable to write data")?;
            }
            "rename" => {
                let name = step.get_req_str("from", i)?;
                self.show_status(&step_type, &name);

                let from = self.dir.join(&name);
                let to = self.dir.join(&step.get_req_str("to", i)?);

                fs::rename(&from, &to).map_err(|_| "Unable to rename file")?;
            }
            "copy" => {
                let name = step.get_req_str("from", i)?;
                self.show_status(&step_type, &name);

                let from = self.dir.join(&name);
                let to = self.dir.join(&step.get_req_str("to", i)?);

                fs::copy(&from, &to).map_err(|_| "Unable to copy file")?;
            }
            "delete" => {
                let name = step.get_req_str("file", i)?;
                let recursive = step.get_opt_in_bool("recursive", i)?;
                let path = Path::new(name);

                if !path.is_dir() {
                    fs::remove_file(name).map_err(|_| "Unable to delete file")?;
                } else if recursive {
                    fs::remove_dir_all(path)
                        .map_err(|_| "Unable to recursively delete directory")?;
                } else {
                    if let Ok(mut contents) = path.read_dir() {
                        if contents.next().is_some() {
                            return Err("Directory is not empty".into());
                        }
                    }

                    fs::remove_dir(path).map_err(|_| "Unable to delete directory")?;
                }
            }
            "deploy" => {
                let name = step.get_req_str("from", i)?;
                self.show_status(&step_type, &name);

                let mut from = self.dir.join(&name);
                let from_is_dir = Path::is_dir(&from);

                let to = step.get_req_str("to", i)?;
                let (host, to_path) = Self::parse_scp_path(&to);

                if step.get_opt_in_bool("clear", i)? {
                    let type_str = if from_is_dir { "directory" } else { "file" };
                    let date_str = SystemTime::now()
                        .duration_since(UNIX_EPOCH)?
                        .as_millis()
                        .to_string();
                    let temp_to = to.to_owned() + "-" + &date_str;
                    let temp_to_path = to_path.to_owned() + "-" + &date_str;

                    Self::deploy_scp(from, &temp_to, name)?;

                    let temp_result = (|| -> Result<(), Box<dyn Error>> {
                        if Self::run_remote_command(
                            host,
                            &format!("rm -rf \"{}\" || del /f /q \"{}\"", to_path, to_path),
                        )?
                        .is_none()
                        {
                            println!("\nCould not remove old deployment {}", to_path)
                        } else {
                            println!("\nRemoved old deployment {}", to_path)
                        }

                        Self::run_remote_command(
                            host,
                            &format!("scp -r \"{}\" \"{}\"", temp_to_path, to_path),
                        )?
                        .ok_or_else(|| {
                            format!(
                                "Failed to move temporary {} {} to {}",
                                type_str, temp_to_path, to_path
                            )
                        })?;

                        Ok(())
                    })();

                    let cleanup_result = Self::run_remote_command(
                        host,
                        &format!(
                            "rm -rf \"{}\" || del /f /q \"{}\"",
                            temp_to_path, temp_to_path
                        ),
                    )?;

                    match cleanup_result {
                        Some(..) => temp_result?,
                        None => {
                            if let Err(err) = temp_result {
                                eprintln!("\n{}", err);
                            }

                            return Err(format!(
                                "Failed to remove temporary {} {}",
                                type_str, temp_to_path
                            )
                            .into());
                        }
                    }
                } else {
                    // Prevent creation of another directory inside existing directory
                    if from_is_dir {
                        Self::run_remote_command(host, &format!("mkdir \"{}\"", to_path))?;
                        from = from.join("*");
                    }

                    Self::deploy_scp(from, to, name)?;
                }
            }
            x => return Err(format!("Unknown type \"{}\" for instruction #{}", x, i).into()),
        }

        return Ok(());
    }
}
