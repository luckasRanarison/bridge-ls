use crate::utils::expand_args;
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    error::Error,
    io::{Seek, SeekFrom, Write},
    path::Path,
    process::{Command, Stdio},
};
use tempfile::tempfile;

const BUILTIN_FORMATTERS: &str = include_str!("../builtins/formatters.json");

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct Config {
    #[serde(default)]
    pub builtins: Builtin,
    #[serde(default)]
    pub customs: Custom,
    #[serde(skip)]
    builtin_formatters: HashMap<String, FormatterConfig>,
    // builtin_linters: HashMap<String, LinterConfig>,
}

impl Config {
    pub fn register_builtins(mut self) -> Self {
        let builtin_formatters: HashMap<String, FormatterConfig> =
            serde_json::from_str(BUILTIN_FORMATTERS).unwrap();
        let filtered = builtin_formatters
            .into_iter()
            .filter(|(name, _)| self.builtins.formatters.contains(name));
        self.builtin_formatters.extend(filtered);
        self
    }

    pub fn get_formatter(&self, extension: &str) -> Option<&FormatterConfig> {
        self.builtin_formatters
            .values()
            .chain(self.customs.formatters.values())
            .find(|f| f.filetypes.iter().any(|e| e == extension))
    }
}

#[derive(Debug, Default, Deserialize, Serialize, PartialEq)]
pub struct Builtin {
    #[serde(default)]
    pub formatters: Vec<String>,
    // linters: Vec<String>,
}

#[derive(Debug, Default, Deserialize, Serialize, PartialEq)]
pub struct Custom {
    #[serde(default)]
    pub formatters: HashMap<String, FormatterConfig>,
    // linters: HashMap<String, LinterConfig>,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct FormatterConfig {
    pub command: String,
    pub args: Vec<String>,
    pub filetypes: Vec<String>,
    pub to_stdin: bool,
    pub cleanup: Option<String>,
}

impl FormatterConfig {
    pub fn format(
        &self,
        document: &str,
        file_path: &Path,
    ) -> Result<String, Box<dyn Error + Send + Sync>> {
        let mut file = tempfile()?;
        file.write_all(document.as_bytes())?;
        file.seek(SeekFrom::Start(0))?;
        let output = Command::new(&self.command)
            .args(expand_args(&self.args, file_path))
            .stdin(Stdio::from(file))
            .output()?;
        let result = String::from_utf8(output.stdout)?;

        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_parsing() {
        let config = r#"{
            "builtins": {
                "formatters": ["prettierd"]
            },
            "customs": {
                "formatters": {
                    "stylua": {
                        "command": "stylua",
                        "args": ["-"],
                        "filetypes": ["lua"],
                        "toStdin": true
                    }
                }
            }
        }"#;
        let parsed: Config = serde_json::from_str(config).unwrap();
        let expected = Config {
            builtins: Builtin {
                formatters: vec!["prettierd".to_owned()],
            },
            customs: Custom {
                formatters: [(
                    "stylua".to_owned(),
                    FormatterConfig {
                        command: "stylua".to_owned(),
                        args: vec!["-".to_owned()],
                        filetypes: vec!["lua".to_owned()],
                        to_stdin: true,
                        cleanup: None,
                    },
                )]
                .into(),
            },
            builtin_formatters: HashMap::default(),
        };

        assert_eq!(parsed, expected);
    }
}
