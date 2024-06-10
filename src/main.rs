use regex::Regex;
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
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
        let reader = io::BufReader::new(file);
        for line in reader.lines() {
            let line = line?;
            self.parse_line(&line);
        }
        Ok(())
    }
}

fn main() -> io::Result<()> {
    let mut config = SysctlConfig::new();
    config.load("sysctl.conf")?;
    println!("{:#?}", config);
    Ok(())
}
