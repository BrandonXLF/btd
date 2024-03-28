use std::error::Error;

use serde::Serialize;
use serde_yaml::{Mapping, Value};

pub type Transformation = Mapping;
trait InternalTrait<T, V> {
    fn get_raw_value(&self, key: &str, i: usize) -> Result<&V, Box<dyn Error>>;
}

pub trait TransformationTrait<T, V> {
    fn get_opt_str(&self, key: &str, i: usize) -> Result<Option<&str>, Box<dyn Error>>;
    fn get_req_str(&self, key: &str, i: usize) -> Result<&str, Box<dyn Error>>;
    fn get_opt_in_bool(&self, key: &str, i: usize) -> Result<bool, Box<dyn Error>>;
    fn get_opt_map(&self, key: &str, i: usize) -> Result<Option<&Mapping>, Box<dyn Error>>;
}

impl InternalTrait<Transformation, Value> for Transformation {
    fn get_raw_value(&self, key: &str, i: usize) -> Result<&Value, Box<dyn Error>> {
        self.get(key)
            .ok_or_else(|| format!("Missing key \"{}\" for ingredient #{}", key, i + 1).into())
    }
}

impl TransformationTrait<Transformation, Value> for Transformation {
    fn get_opt_str(&self, key: &str, i: usize) -> Result<Option<&str>, Box<dyn Error>> {
        match self.get_raw_value(key, i) {
            Ok(raw) => Ok(Some(raw.as_str().ok_or_else(|| -> Box<dyn Error> {
                format!(
                    "Expected \"{}\" to be a string for ingredient #{}",
                    key,
                    i + 1
                )
                .into()
            })?)),
            Err(_) => Ok(None),
        }
    }

    fn get_req_str(&self, key: &str, i: usize) -> Result<&str, Box<dyn Error>> {
        self.get_opt_str(key, i)?.ok_or_else(|| {
            format!(
                "Missing required string \"{}\" for ingredient #{}",
                key,
                i + 1
            )
            .into()
        })
    }

    fn get_opt_in_bool(&self, key: &str, i: usize) -> Result<bool, Box<dyn Error>> {
        match self.get_raw_value(key, i) {
            Ok(raw) => Ok(raw.as_bool().ok_or_else(|| {
                format!(
                    "Expected \"{}\" to be a boolean for ingredient #{}",
                    key,
                    i + 1
                )
            })?),
            Err(_) => Ok(false),
        }
    }

    fn get_opt_map(&self, key: &str, i: usize) -> Result<Option<&Mapping>, Box<dyn Error>> {
        match self.get_raw_value(key, i) {
            Ok(raw) => Ok(Some(raw.as_mapping().ok_or_else(
                || -> Box<dyn Error> {
                    format!(
                        "Expected \"{}\" to be a map for ingredient #{}",
                        key,
                        i + 1
                    )
                    .into()
                },
            )?)),
            Err(_) => Ok(None),
        }
    }
}

#[derive(Serialize)]
pub struct MetaTransformation {
    #[serde(rename = "type")]
    pub _type: String,
    pub dir: String,
}

impl MetaTransformation {
    pub fn new(name: String) -> MetaTransformation {
        MetaTransformation {
            _type: "meta".to_string(),
            dir: name,
        }
    }
}
