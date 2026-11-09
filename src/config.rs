use std::{
    fs,
    path::{Path, PathBuf},
};

use serde::{Deserialize, Serialize};

use crate::error::TDAppError;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default, deny_unknown_fields)]
pub struct AppConfig {
    pub profile_directory: PathBuf,
    pub ryzen_adj_exec_path: PathBuf,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            profile_directory: PathBuf::from("./profiles"),
            ryzen_adj_exec_path: PathBuf::from(
                "./external/ryzenadj-manylinux_2_28-x86_64/ryzenadj",
            ),
        }
    }
}

impl AppConfig {
    pub fn load_or_create(path: &Path) -> Result<Self, TDAppError> {
        if path.exists() {
            return Ok(serde_json::from_str(&fs::read_to_string(path)?)?);
        }

        let config = Self::default();
        fs::write(path, serde_json::to_string_pretty(&config)?)?;
        Ok(config)
    }
}

#[cfg(test)]
mod tests {
    use std::{
        fs,
        path::PathBuf,
        time::{SystemTime, UNIX_EPOCH},
    };

    use super::AppConfig;

    fn temporary_file() -> PathBuf {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        std::env::temp_dir().join(format!(
            "td-pilot-config-test-{}-{timestamp}.json",
            std::process::id()
        ))
    }

    #[test]
    fn creates_default_config_and_reads_it_back() {
        let path = temporary_file();
        let config = AppConfig::load_or_create(&path).unwrap();
        assert_eq!(config.profile_directory, PathBuf::from("./profiles"));
        assert_eq!(
            config.ryzen_adj_exec_path,
            PathBuf::from("./external/ryzenadj-manylinux_2_28-x86_64/ryzenadj")
        );
        assert_eq!(
            AppConfig::load_or_create(&path).unwrap().profile_directory,
            config.profile_directory
        );
        fs::remove_file(path).unwrap();
    }
}
