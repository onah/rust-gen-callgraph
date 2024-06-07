use std::error;
//use std::ffi::OsString;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ErrorKind {
    #[error("Invalid package name in cargo.toml")]
    InvalidPackageName,
}

pub struct ProjectInfomaion {
    project_name: String,
}

impl ProjectInfomaion {
    pub fn new(project_path: &PathBuf) -> Result<ProjectInfomaion, Box<dyn error::Error>> {
        let cargo_file = project_path.join("Cargo.toml");
        let mut f = File::open(&cargo_file)?;
        let mut contents = String::new();
        f.read_to_string(&mut contents)?;

        let project_name = Self::get_project_name_from_cargo_toml(&contents)?;
        Ok(ProjectInfomaion { project_name })
    }

    pub fn project_name(&self) -> &str {
        self.project_name.as_str()
    }

    fn get_project_name_from_cargo_toml(
        cargo_toml_content: &str,
    ) -> Result<String, Box<dyn error::Error>> {
        let values = cargo_toml_content.parse::<toml::Value>()?;
        // TODO: Handle error
        // panic if package name is not found
        let project_name = values["package"]["name"]
            .as_str()
            .ok_or(ErrorKind::InvalidPackageName)?;

        Ok(String::from(project_name))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_project_name_from_cargo_toml() {
        let cargo_toml_content = r#"
            [package]
            name = "test"
            version = "0.1.0"
        "#;
        let project_name = ProjectInfomaion::get_project_name_from_cargo_toml(cargo_toml_content)
            .expect("Failed to get project name from cargo.toml");
        assert_eq!(project_name, "test");
    }

    /*
    #[test]
    fn test_get_project_name_from_cargo_toml_invalid_package_name() {
        let cargo_toml_content = r#"
            [package]
            version = "0.1.0"
        "#;
        let project_name = ProjectInfomaion::get_project_name_from_cargo_toml(cargo_toml_content);
        assert!(project_name.is_err());
    }
    */
}
