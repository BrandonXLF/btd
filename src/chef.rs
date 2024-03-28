use std::{
    error::Error,
    fs,
    io::Write,
    path::Path,
    process::{Command, Output},
};

use system::{system_output, System};

use crate::{
    read::read_recipe,
    transformation::{Transformation, TransformationTrait},
};

static STAGES: &[char] = &['🥚', '🐣', '🐤', '🐔'];

pub struct Chef<'a> {
    dir: &'a Path,
    stage: usize,
}

impl Chef<'_> {
    fn new(dir: &str) -> Chef {
        Chef {
            dir: Path::new(dir),
            stage: 0,
        }
    }

    pub fn process_recipe(name: Option<&str>) -> Result<(), Box<dyn Error>> {
        let mut steps = read_recipe(name)?;

        if steps.len() == 0 {
            return Err("Missing meta step".into());
        }

        let meta = steps.remove(0);
        let dir = meta.get_req_str("dir", 0)?;
        let mut output = Chef::new(&dir);

        for (i, step) in steps.iter().enumerate() {
            output.do_step(step, i + 1)?;
        }

        println!("\n🦖 \x1b[38;5;39mdone\x1b[0m");
        Ok(())
    }

    fn show_status(&mut self, step_type: &str, param: &str) {
        let ingredient = STAGES[self.stage];

        self.stage = (self.stage + 1) % STAGES.len();

        println!(
            "\n{} \x1b[38;5;39m{}\x1b[0m \x1b[38;5;248m{}\x1b[0m",
            ingredient, step_type, param
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

    fn run_remote_command(host: Option<&str>, cmd: &str) -> Result<Output, Box<dyn Error>> {
        let out_result = match host {
            Some(host) => Command::new("ssh").args(&["-tq", host, cmd]).output(),
            None => system_output(cmd),
        };

        Ok(out_result.map_err(|_| "Failed to execute pre-deployment command")?)
    }

    fn do_step(&mut self, step: &Transformation, i: usize) -> Result<(), Box<dyn Error>> {
        let step_type = step.get_req_str("type", i)?;

        match step_type {
            "run" => {
                let cmd = step.get_req_str("cmd", i)?;
                self.show_status(&step_type, &cmd);

                let cwd = step
                    .get_opt_str("cwd", i)?
                    .map(Path::new)
                    .unwrap_or(self.dir);

                let success = Command::system(&cmd)
                    .current_dir(cwd)
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
                fs::remove_file(name).map_err(|_| "Unable to delete file")?;
            }
            "deploy" => {
                let name = step.get_req_str("from", i)?;
                self.show_status(&step_type, &name);

                let mut from = self.dir.join(&name);
                let from_is_dir = Path::is_dir(&from);

                let to = step.get_req_str("to", i)?;
                let (host, to_path) = Self::parse_scp_path(&to);

                if step.get_opt_in_bool("clear", i)? {
                    let output = Self::run_remote_command(
                        host,
                        &format!(
                            "(rm -rf \"{}\" && echo OK) || (del /f /q \"{}\" && echo OK)",
                            to_path, to_path
                        ),
                    )?;

                    let stdout = String::from_utf8_lossy(&output.stdout);
                    let type_str = if from_is_dir { "directory" } else { "file " };

                    if stdout.trim_end() == "OK" {
                        println!("\nRemoved {} {}", type_str, to);
                    } else {
                        return Err(
                            format!("Failed to remove {} before deployment", type_str).into()
                        );
                    }
                }

                if from_is_dir {
                    Self::run_remote_command(host, &format!("mkdir \"{}\"", to_path))?;
                    from = from.join("*");
                }

                println!();

                let success = Command::new("scp")
                    .args(&["-r", &from.to_string_lossy(), &to])
                    .status()
                    .map_err(|_| "Failed to execute scp")?
                    .success();

                if !success {
                    return Err(format!("Failed to transfer {} to {}", name, to).into());
                }
            }
            _ => {}
        }

        return Ok(());
    }
}
