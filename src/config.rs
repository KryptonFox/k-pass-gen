use serde::{Deserialize, Serialize};
use std::fs;
use std::fs::File;
use std::io::{Read, Write};

const DIR_NAME: &str = "KPassGen";
const FILE_NAME: &str = "config.toml";

#[derive(Clone, Serialize, Deserialize)]
pub struct Config {
    pub len: usize,
    pub charset: String,
    pub use_capitals: bool,
}

impl Config {
    pub fn new() -> Self {
        ConfigFile::load().unwrap_or_default().config
    }

    pub fn save(&self) {
        let _ = ConfigFile::from(self).save();
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            len: 8,
            charset: "abcdefghijklmnopqrstuvwxyz0123456789([{?*&%$#@}])".to_string(),
            use_capitals: true,
        }
    }
}

#[derive(Serialize, Deserialize, Default)]
struct ConfigFile {
    pub config: Config,
}

impl From<&Config> for ConfigFile {
    fn from(config: &Config) -> Self {
        Self {
            config: config.clone(),
        }
    }
}

impl ConfigFile {
    pub fn load() -> std::io::Result<Self> {
        let cfg = ConfigFile::default();
        let cfg_dir = dirs::config_dir().ok_or(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "Cannot get config directory",
        ))?;
        let path = cfg_dir.join(DIR_NAME).join(FILE_NAME);

        if !path.exists() {
            cfg.save()?;
            Ok(cfg)
        } else {
            let mut file = File::open(&path)?;
            let mut contents = String::new();
            file.read_to_string(&mut contents)?;

            let cfg = toml::from_str::<Self>(contents.as_str()).map_err(|e| {
                std::io::Error::new(std::io::ErrorKind::InvalidData, format!("{}", e))
            })?;
            Ok(cfg)
        }
    }

    pub fn save(&self) -> std::io::Result<()> {
        let cfg_dir = dirs::config_dir().ok_or(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "Cannot get config directory",
        ))?;
        let path = cfg_dir.join(DIR_NAME);

        if !path.exists() {
            fs::create_dir(path.clone())?;
        }

        let mut file = File::create(path.join(FILE_NAME))?;
        file.write_all(toml::to_string_pretty(self).unwrap().as_bytes())?;

        Ok(())
    }
}
