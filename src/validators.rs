use std::path::Path;

use coordinate_transformer::JprOrigin;
use inquire::CustomUserError;
use inquire::list_option::ListOption;
use inquire::validator::Validation;
use las::Reader;

use crate::file_path_completer::to_plain_text;
use crate::las_info::ZoomLevel;

pub fn is_las_or_laz(path: &str) -> Result<Validation, CustomUserError> {
    let path = to_plain_text(path);

    if Reader::from_path(&path).is_ok() {
        Ok(Validation::Valid)
    } else {
        Ok(Validation::Invalid(format!("\"{}\" is not a LAS or LAZ file", path).into()))
    }
}

pub fn is_exist(path: &str) -> Result<Validation, CustomUserError> {
    let exist = Path::new(&to_plain_text(path)).try_exists().unwrap_or(false);

    if exist {
        Ok(Validation::Valid)
    } else {
        Ok(Validation::Invalid(format!("\"{}\" does not exist", path).into()))
    }
}

pub fn is_jpr_origin(origin: &str) -> Result<Validation, CustomUserError> {
    match origin.parse::<u8>() {
        Ok(num) =>
            if JprOrigin::One as u8 <= num && num <= JprOrigin::Nineteen as u8 {
                Ok(Validation::Valid)
            } else {
                Ok(Validation::Invalid("The input value must be a one-byte number from 1 to 19.".to_string().into()))
            },
        Err(_) => Ok(Validation::Invalid(format!("\"{}\" is not an integer", origin).into())),
    }
}

pub fn zoom_level_selection_validator(input: &[ListOption<&ZoomLevel>]) -> Result<Validation, CustomUserError> {
    if input.is_empty() {
        Ok(Validation::Invalid("Please select at least one zoom level.".into()))
    } else {
        Ok(Validation::Valid)
    }
}
