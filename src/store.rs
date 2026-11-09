use std::{
    fs,
    path::{Path, PathBuf},
};

use crate::{
    error::{TDAppError, TDCustomError},
    model::{Profile, RyzenAdjConfig},
};

#[derive(Debug, Clone, Default)]
pub struct Store {
    pub path: PathBuf,
}

impl Store {
    pub fn parse_profile(&self, mut profile: Profile) -> Result<Profile, TDAppError> {
        let file_path = self.get_file_path(&profile.name)?;
        let json_string = fs::read_to_string(file_path)?;
        let value = serde_json::from_str::<RyzenAdjConfig>(&json_string)?;
        profile.config = Some(value);
        Ok(profile)
    }

    pub fn save_profile(&self, profile: Profile, force: bool) -> Result<(), TDAppError> {
        let file_path = self.profile_path(&profile.name)?;
        Self::ensure_directory_exists(&self.path)?;
        if file_path.exists() && !force {
            return Err(TDCustomError {
                message: format!(
                    "Profile '{}' already exists; use --force to replace it.",
                    profile.name
                )
            }.into());
        }
        fs::write(file_path, serde_json::to_string_pretty(&profile.config)?)?;
        Ok(())
    }

    pub fn list_profiles(&self) -> Result<Vec<String>, TDAppError> {
        Self::validate_dir_path(&self.path)?;
        let mut profiles = Vec::<String>::new();
        let entries = fs::read_dir(&self.path)?;

        for entry in entries {
            let dir_entry = entry?;
            let path = dir_entry.path();
            if path.is_file()
                && path
                    .extension()
                    .is_some_and(|extension| extension == "json")
                && let Some(name) = path.file_stem()
            {
                profiles.push(name.to_string_lossy().into_owned());
            }
        }

        profiles.sort();
        Ok(profiles)
    }

    pub fn delete_profile(&self, profile: Profile) -> Result<(), TDAppError> {
        let file_path = self.get_file_path(&profile.name)?;
        fs::remove_file(file_path)?;
        Ok(())
    }

    fn validate_dir_path(path: &Path) -> Result<(), TDCustomError> {
        if !path.exists() || !path.is_dir() {
            return Err(TDCustomError {
                message: format!(
                    "Profile directory '{}' does not exist or is invalid.",
                    path.display()
                ),
            });
        }
        Ok(())
    }

    fn profile_path(&self, name: &str) -> Result<PathBuf, TDCustomError> {
        let path = Path::new(name);
        if path.components().count() != 1
            || !matches!(
                path.components().next(),
                Some(std::path::Component::Normal(_))
            )
            || name == "."
            || name == ".."
        {
            return Err(TDCustomError {
                message: "Profile name must be a file name, not a path.".to_owned(),
            });
        }

        let mut file_name = PathBuf::from(name);
        if path.extension().is_none() {
            file_name.set_extension("json");
        } else if path
            .extension()
            .is_some_and(|extension| extension != "json")
        {
            return Err(TDCustomError {
                message: "Profile files must use the .json extension.".to_owned(),
            });
        }

        Ok(self.path.join(file_name))
    }

    fn get_file_path(&self, name: &str) -> Result<PathBuf, TDCustomError> {
        Self::validate_dir_path(&self.path)?;
        let file_path = self.profile_path(name)?;
        Self::validate_file_path(&file_path)?;
        Ok(file_path)
    }

    fn validate_file_path(path: &Path) -> Result<(), TDCustomError> {
        if !path.exists() || !path.is_file() {
            return Err(TDCustomError {
                message: format!(
                    "Profile file '{}' does not exist or is invalid.",
                    path.display()
                ),
            });
        }
        Ok(())
    }

    fn ensure_directory_exists(path: &Path) -> Result<(), TDAppError> {
        if !path.exists() {
            fs::create_dir_all(path)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::{
        fs,
        path::PathBuf,
        time::{SystemTime, UNIX_EPOCH},
    };

    use super::Store;
    use crate::model::{Profile, RyzenAdjConfig};

    fn temporary_directory() -> PathBuf {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        std::env::temp_dir().join(format!("td-pilot-test-{}-{timestamp}", std::process::id()))
    }

    #[test]
    fn saves_lists_loads_and_deletes_profiles() {
        let directory = temporary_directory();
        let store = Store {
            path: directory.clone(),
        };
        let profile = Profile {
            name: "balanced".to_owned(),
            config: Some(RyzenAdjConfig {
                stapm_limit: Some(12000),
                ..Default::default()
            }),
        };

        store.save_profile(profile.clone(), false).unwrap();
        assert_eq!(store.list_profiles().unwrap(), vec!["balanced"]);
        assert_eq!(
            store
                .parse_profile(Profile {
                    name: "balanced".to_owned(),
                    config: None
                })
                .unwrap()
                .config
                .unwrap()
                .stapm_limit,
            Some(12000)
        );
        assert!(store.save_profile(profile.clone(), false).is_err());
        store.save_profile(profile.clone(), true).unwrap();
        store.delete_profile(profile).unwrap();

        fs::remove_dir_all(directory).unwrap();
    }

    #[test]
    fn rejects_profile_paths() {
        let store = Store {
            path: temporary_directory(),
        };
        assert!(store.profile_path("../outside").is_err());
        assert!(store.profile_path("profile.txt").is_err());
    }
}
