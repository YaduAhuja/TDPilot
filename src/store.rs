use std::{fs, path::{Path, PathBuf}};

use crate::{error::{TDAppError, TDCustomError}, model::{Profile, RyzenAdjConfig}};

#[derive(Debug, Clone, Default)]
pub struct Store {
    pub path: String
}

impl Store {
    pub fn parse_profile(&self, mut profile: Profile) -> Result<Profile, TDAppError> {
        let file_path = self.get_file_path(&profile.name)?;
        let json_string = fs::read_to_string(file_path)?;
        let value = serde_json::from_str::<RyzenAdjConfig>(&json_string)?;
        profile.config = Some(value);
        Ok(profile)
    }


    pub fn save_profile(&self, profile: Profile) -> Result<(), TDAppError> {
        let dir_path = Path::new(&self.path);
        Self::ensure_directory_exists(dir_path)?;
        let mut file_name = PathBuf::from(&profile.name);
        if !profile.name.ends_with(".json"){
            file_name.set_extension("json");
        }
        let file_path = dir_path.join(file_name);
        fs::write(file_path, serde_json::to_string_pretty(&profile.config)?)?;
        Ok(())
    }

    pub fn list_profiles(&self) -> Result<Vec<String>, TDAppError> {
        let path = Path::new(&self.path);
        Self::validate_dir_path(path)?;
        let mut profiles = Vec::<String>::new();
        let entries = fs::read_dir(path)?;

        for entry in entries {
            if let Some(dir_entry) = entry.ok() {
                profiles.push(dir_entry.file_name().into_string().expect("Unable to parse profile name"));
            }
        }

        Ok(profiles)
    }

    pub fn delete_profile(&self, profile: Profile) -> Result<(), TDAppError> {
        let file_path = self.get_file_path(&profile.name)?;
        fs::remove_file(file_path)?;
        Ok(())
    }

    fn validate_dir_path(path: &Path) -> Result<(), TDCustomError> {
        if !path.exists() || !path.is_dir() {
            return Err(TDCustomError { message: "Directory does not exists or is invalid."});
        }
        Ok(())
    }

    fn get_file_path(&self, name: &str) -> Result<PathBuf, TDCustomError> {
        let dir_path = Path::new(&self.path);
        Self::validate_dir_path(dir_path)?;

        let mut file_name = PathBuf::from(name);
        if !name.ends_with(".json"){
            file_name.set_extension("json");
        }

        let file_path = dir_path.join(file_name);
        Self::validate_file_path(&file_path)?;
        Ok(file_path)
    }

    fn validate_file_path(path: &Path) -> Result<(), TDCustomError> {
        if !path.exists() || !path.is_file() {
            return Err(TDCustomError { message: "File does not exists or is invalid." })
        }
        Ok(())
    }

    fn ensure_directory_exists(path: &Path) -> Result<(), TDAppError> {
        if !path.exists() {
            fs::create_dir(path)?;
        }
        Ok(())
    }
}