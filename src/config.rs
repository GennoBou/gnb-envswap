use serde::Deserialize;
use std::collections::HashMap;
use std::path::PathBuf;
use std::{env, fs};

#[derive(Debug, Deserialize, Clone)]
pub struct EnvValue {
    pub label: String,
    pub value: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct EnvVar {
    #[serde(rename = "values")]
    pub values: Vec<EnvValue>,
}

pub type Config = HashMap<String, EnvVar>;

/// Loads and merges configuration from home and current directories.
/// Work directory's config takes precedence.
pub fn load_config() -> Result<Config, String> {
    let home_path = dirs::home_dir().map(|p| p.join(".env.swap.toml"));
    let work_path = env::current_dir().ok().map(|p| p.join(".env.swap.toml"));

    // If work and home paths are the same, treat as if there's only a work path.
    let (work_path, home_path) = if work_path == home_path {
        (work_path, None)
    } else {
        (work_path, home_path)
    };

    let work_config = read_config_from_path(work_path.as_ref())?;
    let home_config = read_config_from_path(home_path.as_ref())?;

    match (work_config, home_config) {
        (Some(mut work), Some(home)) => {
            // Merge home config into work config.
            for (key, home_var) in home {
                let work_var = work.entry(key).or_insert_with(|| EnvVar { values: Vec::new() });
                work_var.values.extend(home_var.values);
            }
            Ok(work)
        }
        (Some(work), None) => Ok(work),
        (None, Some(home)) => Ok(home),
        (None, None) => Err("No .env.swap.toml file found in current or home directory.".to_string()),
    }
}

/// Reads and parses a config file from a given optional path.
fn read_config_from_path(path: Option<&PathBuf>) -> Result<Option<Config>, String> {
    match path {
        Some(p) if p.exists() => {
            let content = fs::read_to_string(p)
                .map_err(|e| format!("Failed to read config file at {:?}: {}", p, e))?;
            let config: Config = toml::from_str(&content)
                .map_err(|e| format!("Failed to parse TOML at {:?}: {}", p, e))?;
            Ok(Some(config))
        }
        _ => Ok(None),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;
    use std::sync::Mutex;
    use tempfile::tempdir;

    // Mutex to serialize tests that modify the environment (CWD, env vars).
    static ENV_MUTEX: Mutex<()> = Mutex::new(());

    fn create_dummy_config(content: &str) -> (tempfile::TempDir, PathBuf) {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join(".env.swap.toml");
        fs::write(&file_path, content).unwrap();
        (dir, file_path)
    }

    #[test]
    fn test_load_config_no_files() {
        let _lock = ENV_MUTEX.lock().unwrap();
        let dir = tempdir().unwrap();
        env::set_current_dir(dir.path()).unwrap();
        env::set_var("HOME", dir.path()); // Ensure home is also empty

        let result = load_config();
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            "No .env.swap.toml file found in current or home directory."
        );
    }

    #[test]
    fn test_load_config_only_work_dir() {
        let _lock = ENV_MUTEX.lock().unwrap();
        let config_content = r#"
            [API_KEY]
            [[API_KEY.values]]
            label = "Dev"
            value = "dev-key"
        "#;
        let (dir, _config_path) = create_dummy_config(config_content);
        env::set_current_dir(dir.path()).unwrap();
        env::set_var("HOME", "/tmp/non-existent-home-for-sure");

        let config = load_config().unwrap();
        assert!(config.contains_key("API_KEY"));
        assert_eq!(config["API_KEY"].values.len(), 1);
        assert_eq!(config["API_KEY"].values[0].label, "Dev");
    }

    #[test]
    fn test_load_config_merge_behavior() {
        let _lock = ENV_MUTEX.lock().unwrap();
        let home_content = r#"
            [API_KEY]
            [[API_KEY.values]]
            label = "Home"
            value = "home-key"

            [DB_HOST]
            [[DB_HOST.values]]
            label = "Home DB"
            value = "db.home"
        "#;
        let work_content = r#"
            [API_KEY]
            [[API_KEY.values]]
            label = "Work"
            value = "work-key"
        "#;

        let home_dir = tempdir().unwrap();
        let work_dir = tempdir().unwrap();

        fs::write(home_dir.path().join(".env.swap.toml"), home_content).unwrap();
        fs::write(work_dir.path().join(".env.swap.toml"), work_content).unwrap();

        env::set_current_dir(work_dir.path()).unwrap();
        env::set_var("HOME", home_dir.path().to_str().unwrap());

        let config = load_config().unwrap();

        // API_KEY should have two values, work's coming first
        assert_eq!(config["API_KEY"].values.len(), 2);
        assert_eq!(config["API_KEY"].values[0].label, "Work");
        assert_eq!(config["API_KEY"].values[1].label, "Home");

        // DB_HOST should exist from home config
        assert!(config.contains_key("DB_HOST"));
        assert_eq!(config["DB_HOST"].values.len(), 1);
    }
}
