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

/// Internal logic for loading and merging configuration from given paths.
fn load_config_from_paths(
    work_path: Option<PathBuf>,
    home_path: Option<PathBuf>,
) -> Result<Config, String> {
    // If work and home paths are the same, treat as if there's only a work path.
    let (work_path, home_path) = if work_path == home_path {
        (work_path, None)
    } else {
        (work_path, home_path)
    };

    let mut work_config = read_config_from_path(work_path.as_ref())?;
    let mut home_config = read_config_from_path(home_path.as_ref())?;

    // Prepend prefixes to labels to indicate source.
    if let Some(work) = work_config.as_mut() {
        for var in work.values_mut() {
            for val in var.values.iter_mut() {
                val.label = format!("<Work> {}", val.label);
            }
        }
    }
    if let Some(home) = home_config.as_mut() {
        for var in home.values_mut() {
            for val in var.values.iter_mut() {
                val.label = format!("<Home> {}", val.label);
            }
        }
    }

    match (work_config, home_config) {
        (Some(mut work), Some(home)) => {
            // Merge home config into work config.
            for (key, home_var) in home {
                let work_var = work.entry(key).or_insert_with(|| EnvVar {
                    values: Vec::new(),
                });
                work_var.values.extend(home_var.values);
            }
            Ok(work)
        }
        (Some(work), None) => Ok(work),
        (None, Some(home)) => Ok(home),
        (None, None) => {
            Err("No .env.swap.toml file found in current or home directory.".to_string())
        }
    }
}

/// Loads and merges configuration from home and current directories.
/// Work directory's config takes precedence.
pub fn load_config() -> Result<Config, String> {
    let home_path = dirs::home_dir().map(|p| p.join(".env.swap.toml"));
    let work_path = env::current_dir().ok().map(|p| p.join(".env.swap.toml"));

    load_config_from_paths(work_path, home_path)
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

    #[test]
    fn test_load_config_no_files() {
        let result = load_config_from_paths(None, None);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            "No .env.swap.toml file found in current or home directory."
        );
    }

    #[test]
    fn test_load_config_only_work_dir() {
        let dir = tempfile::tempdir().unwrap();
        let work_path = dir.path().join(".env.swap.toml");
        let config_content = r#"
            [API_KEY]
            [[API_KEY.values]]
            label = "Dev"
            value = "dev-key"
        "#;
        fs::write(&work_path, config_content).unwrap();

        let config = load_config_from_paths(Some(work_path), None).unwrap();
        assert!(config.contains_key("API_KEY"));
        assert_eq!(config["API_KEY"].values.len(), 1);
        assert_eq!(config["API_KEY"].values[0].label, "<Work> Dev");
    }

    #[test]
    fn test_load_config_merge_behavior() {
        let home_dir = tempfile::tempdir().unwrap();
        let work_dir = tempfile::tempdir().unwrap();
        let home_path = home_dir.path().join(".env.swap.toml");
        let work_path = work_dir.path().join(".env.swap.toml");

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

        fs::write(&home_path, home_content).unwrap();
        fs::write(&work_path, work_content).unwrap();

        let config = load_config_from_paths(Some(work_path), Some(home_path)).unwrap();

        // API_KEY should have two values, work's coming first
        assert_eq!(config["API_KEY"].values.len(), 2);
        assert_eq!(config["API_KEY"].values[0].label, "<Work> Work");
        assert_eq!(config["API_KEY"].values[1].label, "<Home> Home");

        // DB_HOST should exist from home config
        assert!(config.contains_key("DB_HOST"));
        assert_eq!(config["DB_HOST"].values.len(), 1);
        assert_eq!(config["DB_HOST"].values[0].label, "<Home> Home DB");
    }
}
