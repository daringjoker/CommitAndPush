use core::panic;
use serde::{de::DeserializeOwned, Deserialize};
use std::{
    env, fs,
    path::{self, PathBuf},
    process::Command,
};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LocalConfig {
    pub prefix: Option<String>,
    pub suffix: Option<String>,
    pub base_branch: Option<String>,
    pub default_prompt: Option<String>,
    #[serde(flatten)]
    pub categories: std::collections::HashMap<String, CategoryConfig>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Config {
    pub prefix: String,
    pub suffix: String,
    pub base_branch: String,
    pub default_prompt: String,
    #[serde(flatten)]
    pub categories: std::collections::HashMap<String, CategoryConfig>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CategoryConfig {
    desc: String,
    prompt: String,
}

impl Config {
    pub fn parse() -> Config {
        let home_path = env::var("HOME").unwrap();
        let config_file_path = path::Path::new(&home_path).join(".cnp.toml");
        let config: Config = Config::parse_from_path(&config_file_path);

        let top_level_dir_cmd = Command::new("git")
            .args(["rev-parse", "--show-toplevel"])
            .output()
            .expect("failed to execute process");

        let top_level_dir = String::from_utf8(top_level_dir_cmd.stdout).unwrap();
        let config_file_path = path::Path::new(&top_level_dir.trim()).join(".cnp.toml");
        if config_file_path.exists() {
            let local_config: LocalConfig = Config::parse_from_path(&config_file_path);
            return Config::merge_configs(config, local_config);
        }
        return config;
    }
    fn merge_configs(config: Config, local_config: LocalConfig) -> Config {
        let mut merged_config = config;

        if let Some(prefix) = local_config.prefix {
            merged_config.prefix = prefix;
        }
        if let Some(suffix) = local_config.suffix {
            merged_config.suffix = suffix;
        }
        if let Some(base_branch) = local_config.base_branch {
            merged_config.base_branch = base_branch;
        }
        if let Some(default_prompt) = local_config.default_prompt {
            merged_config.default_prompt = default_prompt;
        }
        for (key, value) in local_config.categories {
            merged_config.categories.insert(key, value);
        }
        return merged_config;
    }

    fn parse_from_path<ConfigType: DeserializeOwned>(config_file_path: &PathBuf) -> ConfigType {
        if !fs::exists(&config_file_path).unwrap() {
            println!("Config file not found at: {:?}", config_file_path);
            panic!("Config file not found");
        }

        let config = fs::read_to_string(config_file_path);
        if config.is_err() {
            println!("Error reading config file: {:?}", config.err());
            panic!("Failed to read config file");
        }

        let copied_config_content: Result<ConfigType, toml::de::Error> =
            toml::from_str(&config.unwrap());

        if copied_config_content.is_err() {
            println!(
                "Error Parsing Config file.\n{:?}",
                copied_config_content.err()
            );
            panic!("Failed to read config file");
        }

        return copied_config_content.unwrap();
    }

    pub fn has_categories(&self) -> bool {
        return !self.categories.is_empty();
    }

    pub fn get_category_determination_prompt(&self) -> String {
        let category_prompt_details = self
            .categories
            .iter()
            .map(|(key, value)| {
                return format!("{} -> {}\n", key, value.desc);
            })
            .collect::<String>();

        format!(
            "What category does this PR belong to?\n{}reply with the category name only",
            category_prompt_details
        )
    }

    pub fn get_prompt_for_category(&self, category: &str) -> String {
        let category_config = self.categories.get(category);
        if category_config.is_none() {
            return self.default_prompt.clone();
        }
        let category_config = category_config.unwrap();
        return format!(
            "{}\n{}\n{}",
            self.prefix, category_config.prompt, self.suffix
        );
    }
}
