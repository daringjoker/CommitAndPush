use core::panic;
use serde::Deserialize;
use std::{env, fs, path};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Config {
    pub prefix: String,
    pub suffix: String,
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
        if !fs::exists(&config_file_path).unwrap() {
            println!("Config file not found at: {:?}", config_file_path);
            panic!("Config file not found");
        }

        let config = fs::read_to_string(config_file_path);
        if config.is_err() {
            println!("Error reading config file: {:?}", config.err());
            panic!("Failed to read config file");
        }

        let copied_config_content: Result<Config, toml::de::Error> =
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
