use std::fs::read_to_string;
use std::path::PathBuf;

use yaml_rust::{Yaml, YamlLoader};

pub fn get_yaml<P: Into<PathBuf>>(path: P) -> Option<Yaml> {
    let contents = read_to_string(path.into()).ok()?;
    Some(YamlLoader::load_from_str(&contents).ok()?.get(0)?.clone())
}
