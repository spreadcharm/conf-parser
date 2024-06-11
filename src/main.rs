use regex::Regex;
use serde::Deserialize;
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;

#[derive(Debug)]
struct SysctlConfig {
    settings: HashMap<String, String>,
}

impl SysctlConfig {
    fn new() -> Self {
        SysctlConfig {
            settings: HashMap::new(),
        }
    }

    fn parse_line(&mut self, line: &str) {
        let re = Regex::new(r"^\s*([^#;][^=]+)\s*=\s*(.+)\s*$").unwrap();
        if let Some(caps) = re.captures(line) {
            let key = caps[1].trim().to_string();
            let value = caps[2].trim().to_string();
            self.settings.insert(key, value);
        }
    }

    fn load<P: AsRef<Path>>(&mut self, filename: P) -> io::Result<()> {
        let file = File::open(filename)?;
        let reader = BufReader::new(file);
        for line in reader.lines() {
            let line = line?;
            self.parse_line(&line);
        }
        Ok(())
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
enum ConfigType {
    String,
    Bool,
}

#[derive(Debug, Deserialize)]
struct Schema {
    schema: HashMap<String, ConfigType>,
}

impl Schema {
    fn load<P: AsRef<Path>>(filename: P) -> io::Result<Self> {
        let file = File::open(filename)?;
        let reader = BufReader::new(file);
        let schema: Schema = serde_json::from_reader(reader)?;
        Ok(schema)
    }

    fn validate(&self, config: &SysctlConfig) -> Result<(), String> {
        for (key, value) in &config.settings {
            if let Some(expected_type) = self.schema.get(key) {
                match expected_type {
                    ConfigType::String => {}
                    ConfigType::Bool => {
                        if value != "true" && value != "false" {
                            return Err(format!("Validation error: '{}' should be a boolean", key));
                        }
                    }
                }
            } else {
                return Err(format!("Validation error: unexpected key '{}'", key));
            }
        }
        Ok(())
    }
}

fn main() -> io::Result<()> {
    let mut config = SysctlConfig::new();
    config.load("sysctl.conf")?;

    let schema = Schema::load("schema.json")?;
    match schema.validate(&config) {
        Ok(_) => {
            println!("Configuration is valid:\n{:#?}", config);
        }
        Err(e) => {
            eprintln!("Configuration validation failed: {}", e);
        }
    }

    Ok(())
}
