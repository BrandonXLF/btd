use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct LibraryConfig {
    pub dir: Option<String>,
    pub base: Option<String>,
}

impl LibraryConfig {
    pub fn new_empty() -> LibraryConfig {
        LibraryConfig {
            dir: None,
            base: None,
        }
    }
}
