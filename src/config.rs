use serde::{Deserialize, Serialize};
use std::{collections::HashMap, env, fs};

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct Config {
    formatters: HashMap<String, FormatterOptions>,
    // linters: HashMap<String, LinterOptions>,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct FormatterOptions {
    command: String,
    args: Vec<String>,
    to_stdin: bool,
    cleanup_command: Option<String>,
}

pub fn read_config() -> Config {
    let file_path =
        env::var("BRIDGE_LS_CONFIG").expect("BRIDGE_LS_CONFIG environment variable is not set");
    let buffer =
        fs::read_to_string(file_path).expect("File specified by BRIDGE_LS_CONFIG is not found");

    serde_json::from_str(&buffer).expect("Failed to parse configuration file")
}

#[cfg(test)]
mod tests {
    use super::{Config, FormatterOptions};

    #[test]
    fn test_config_parsing() {
        let config = r#"{
            "formatters": {
                "stylua": {
                    "command": "stylua",
                    "args": ["-"],
                    "toStdin": true
                }
            }
        }"#;
        let parsed: Config = serde_json::from_str(config).unwrap();
        let expected = Config {
            formatters: [(
                "stylua".to_owned(),
                FormatterOptions {
                    command: "stylua".to_owned(),
                    args: vec!["-".to_owned()],
                    to_stdin: true,
                    cleanup_command: None,
                },
            )]
            .into(),
        };

        assert_eq!(parsed, expected);
    }
}
