use std::error;
use std::ffi::OsString;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;
use thiserror::Error;

pub struct VariableDefine {
    name: String,
    // TODO: Option ha huyou ??
    variable_type: Option<String>,
}

impl VariableDefine {
    pub fn new(name: String, variable_type: Option<String>) -> VariableDefine {
        VariableDefine {
            name,
            variable_type,
        }
    }

    pub fn same_name(&self, other: &str) -> bool {
        self.name == other
    }

    pub fn variable_type(&self) -> Option<String> {
        self.variable_type.clone()
    }
}

#[derive(Error, Debug)]
pub enum ErrorKind {
    #[error("Invalid location from lib.rs")]
    InvalidLocLibRs,
    #[error("Invalid package name in cargo.toml")]
    InvalidPackageName,
    #[error("Invalid filename")]
    InvalidFilename,
}

/// Check the module name to which the file belongs.
/// If filename is lib.rs, then we check the Cargo.toml
pub fn get_module_name(file_path: &PathBuf) -> Result<String, Box<dyn error::Error>> {
    let file_name = file_path.file_name().ok_or(ErrorKind::InvalidFilename)?;
    if file_name == OsString::from("lib.rs") {
        // If the file name is lib.rs, there is cargo.toml up two.
        let parent_dir = file_path.parent().ok_or(ErrorKind::InvalidLocLibRs)?;
        let cargo_dir = parent_dir.parent().ok_or(ErrorKind::InvalidLocLibRs)?;
        let cargo_file = cargo_dir.join("Cargo.toml");

        return get_project_name_from_cargo_toml(&cargo_file);
    }

    let result = file_path.file_stem().ok_or(ErrorKind::InvalidFilename)?;
    let result = result.to_str().ok_or(ErrorKind::InvalidFilename)?;

    Ok(result.to_string())
}

fn get_project_name_from_cargo_toml(cargo_file: &PathBuf) -> Result<String, Box<dyn error::Error>> {
    let mut f = File::open(cargo_file)?;
    let mut contents = String::new();
    f.read_to_string(&mut contents)?;

    let values = contents.parse::<toml::Value>()?;
    let project_name = values["package"]["name"]
        .as_str()
        .ok_or(ErrorKind::InvalidPackageName)?;

    Ok(String::from(project_name))
}
