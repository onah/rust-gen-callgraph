use std::error;
//use std::ffi::OsString;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ErrorKind {
    #[error("Invalid location from lib.rs")]
    InvalidLocLibRs,
    #[error("Invalid package name in cargo.toml")]
    InvalidPackageName,
    #[error("Invalid filename")]
    InvalidFilename,
}

pub struct ProjectInfomaion {
    project_name: String,
}

impl ProjectInfomaion {
    pub fn new(project_path: &PathBuf) -> Result<ProjectInfomaion, Box<dyn error::Error>> {
        let cargo_file = project_path.join("Cargo.toml");
        let project_name = Self::get_project_name_from_cargo_toml(&cargo_file)?;
        Ok(ProjectInfomaion { project_name })
    }

    pub fn project_name(&self) -> &str {
        self.project_name.as_str()
    }

    fn get_project_name_from_cargo_toml(
        cargo_file: &PathBuf,
    ) -> Result<String, Box<dyn error::Error>> {
        let mut f = File::open(cargo_file)?;
        let mut contents = String::new();
        f.read_to_string(&mut contents)?;

        let values = contents.parse::<toml::Value>()?;
        let project_name = values["package"]["name"]
            .as_str()
            .ok_or(ErrorKind::InvalidPackageName)?;

        Ok(String::from(project_name))
    }
}
