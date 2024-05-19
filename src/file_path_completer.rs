use std::cmp::PartialEq;
use std::collections::HashMap;
use std::env::current_dir;
use std::fs::{DirEntry, read_dir};
use std::path::Path;

use inquire::{Autocomplete, CustomUserError};
use inquire::autocompletion::Replacement;
use regex::Regex;

#[derive(Debug, Clone, PartialEq, Eq)]
enum PathType {
    File,
    Directory,
    Symlink,
    Other,
}

#[derive(Debug, Clone)]
struct ValidPath {
    pub full_path: String,
    pub path_type: PathType,
}

type Name = String;

impl ValidPath {
    pub fn from_dir_entry(entry: DirEntry, file_validator: Regex) -> Option<(Name, Self)> {
        let path = entry.path();


        let path_type = {
            let file_type = entry.file_type().ok()?;

            if file_type.is_file() {
                PathType::File
            } else if file_type.is_dir() {
                PathType::Directory
            } else if file_type.is_symlink() {
                PathType::Symlink
            } else {
                PathType::Other
            }
        };

        match path_type {
            PathType::File => {
                if !file_validator.is_match(path.to_str()?) {
                    return None;
                }

                let full_path = path.to_str()?.to_string();
                let name = path.file_name()?.to_str()?.to_string();


                Some((name,
                      Self {
                          full_path,
                          path_type,
                      }))
            }
            PathType::Directory => {
                let full_path = path.to_str()?.to_string();
                let name = path.file_name()?.to_str()?.to_string();

                Some((name,
                      Self {
                          full_path,
                          path_type,
                      }))
            }
            PathType::Symlink => {
                let full_path = path.to_str()?.to_string();
                let name = path.file_name()?.to_str()?.to_string();


                Some((name,
                      Self {
                          full_path,
                          path_type,
                      }))
            }
            PathType::Other => None,
        }
    }
}

#[derive(Clone)]
pub struct FilePathCompleter {
    current_dir: String,
    paths: HashMap<String, ValidPath>,
    file_validator: Regex,
}

impl FilePathCompleter {
    pub fn new(file_validator: Regex) -> Self {
        let current_dir = current_dir().unwrap().to_str().unwrap().to_string();

        let mut new_instance = Self {
            current_dir: current_dir.clone(),
            paths: HashMap::new(),
            file_validator,
        };
        new_instance.update(&current_dir);

        new_instance
    }
    pub fn update(&mut self, input: &str) {
        if let Some(current_dir) = Self::current_dir(input) {
            if Path::new(&current_dir).try_exists().is_ok() {
                self.current_dir = current_dir;

                if let Ok(dir) = read_dir(&self.current_dir) {
                    self.paths = dir.filter_map(|e| {
                        let entry = e.ok()?;
                        ValidPath::from_dir_entry(entry, self.file_validator.clone())
                    }).filter(
                        |(_, p)| p.full_path.starts_with(input)
                    ).collect();
                }
            }
        }
    }

    fn current_dir(input: &str) -> Option<String> {
        let input_path = Path::new(input);

        if input.ends_with('/') {
            Some(input.to_string())
        } else if let Some(parent) = input_path.parent() {
            let joined = parent.to_str()?.to_string() + "/";
            Some(joined)
        } else {
            None
        }
    }
}


impl Autocomplete for FilePathCompleter {
    fn get_suggestions(&mut self, input: &str) -> Result<Vec<String>, CustomUserError> {
        self.update(input);
        let mut path_list = self.paths.values().map(|path| {
            // 一致していない部分
            let diff = path.full_path.replace(input, "");

            // 一致している部分
            let matched_name = path.full_path.replace(&diff, "");

            match path.path_type {
                PathType::File => {
                    format!("{}\x1b[32m{}\x1b[39m", matched_name, diff)
                }
                PathType::Symlink => {
                    format!("{}\x1b[36m{}\x1b[39m", matched_name, diff)
                }
                _ => {
                    format!("{}{}", matched_name, diff)
                }
            }
        }).collect::<Vec<_>>();
        path_list.sort();
        Ok(path_list)
    }
    fn get_completion(&mut self, input: &str, highlighted_suggestion: Option<String>) -> Result<Replacement, CustomUserError> {
        self.update(input);

        Ok(match highlighted_suggestion {
            Some(suggestion) => {
                Replacement::Some(to_plain_text(&suggestion))
            }
            None => {
                let mut path_list = self.paths.values().collect::<Vec<_>>();
                path_list.sort_by_key(|path| path.full_path.clone());
                path_list.first().map(|full_path| full_path.full_path.clone())
            }
        })
    }
}

pub fn to_plain_text(s: &str) -> String {
    let escape_sequence = Regex::new(r"\x1b\[[0-9;]*m").unwrap();
    escape_sequence.replace_all(s, "").to_string()
}


